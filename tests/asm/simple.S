addi x1, x0, 6        # Load immediate:  x1 = 6
addi x2, x0, 4        # Load immediate:  x2 = 4

add  x3, x1, x2       # Addition:        x3 = 6 + 4 = 10
sub  x4, x1, x2       # Subtraction:     x4 = 6 - 4 = 2
and  x5, x1, x2       # Bitwise AND:     x5 = 6 & 4 = 4
slli x6, x4, 2        # Shift left:      x6 = 2 << 2 = 8

ebreak