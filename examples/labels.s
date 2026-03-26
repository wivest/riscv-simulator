_start:
    addi a0, zero, 2
    addi a1, zero, 40
    jal ra, add_numbers
    sb a0, 0(zero)
    ebreak

add_numbers:
    add a0, a0, a1
    jalr zero, ra, 0
