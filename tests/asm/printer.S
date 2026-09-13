.equ SERIAL_PORT_BASE,      0xffffc000
.equ SERP_TX_ST_REG_o,          0x0008
.equ SERP_TX_DATA_REG_o,        0x000c

.text

_start:
    la a0, message
    call print_message
    ebreak
    

print_character:
    li t0, SERIAL_PORT_BASE

tx_busy:
    lw   t1, SERP_TX_ST_REG_o(t0)   // read status of transmitter
    andi t1, t1, 0x1                // mask ready bit
    beq  t1, zero, tx_busy          // continue only if ready

    sw   a0, SERP_TX_DATA_REG_o(t0)   // write character to Tx data register
    ret


print_message:
    // save a0, ra
    addi sp, sp, -16
    sw a0, 0(sp)
    sw ra, 4(sp)

    loop:
        lb a0, 0(a0)
        beq a0, zero, end
        call print_character
        lw a0, 0(sp)
        addi a0, a0, 1
        sw a0, 0(sp)
        j loop

    end:
        lw ra, 4(sp)
        addi sp, sp, 16
        ret


.data
.org 0x600
message: .asciz "this is from printer.S file"
