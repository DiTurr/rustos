# trap.S
# In the future our trap vector will go here.

.global asm_trap_vector
asm_trap_vector:
    li a0, 0x10000000       # UART base
    li a1, '!'
1:
    lb t0, 5(a0)            # Check Line Status Register (LSR)
    andi t0, t0, 0x20       # Bit 5: Transmit Holding Register Empty
    beqz t0, 1b
    sb a1, 0(a0)            # Write '!' to UART
2:  j 2b                    # Loop forever
