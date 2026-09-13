.data
.org 0x1000
words:
    .asciz "hello from assembly!"

.text
.org 0x200

_start:
    la a0, words 
    addi a1, zero, ' ' # the character to replace
    addi a2, zero, ',' # the character to replace with
loop:
    lb t0, 0(a0)
    beq t0, zero, end # end of the string
    beq t0, a1, replace
    addi a0, a0, 1
    j loop
replace:
    sb a2, 0(a0)
    addi a0, a0, 1
    j loop
end:
    ebreak
