# Disable generation of compressed instructions.
.option norvc

.section .text
.global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
asm_trap_vector:
	csrr	a0, mepc
	csrr	a1, mtval
	csrr	a2, mcause
	csrr	a3, mhartid
	csrr	a4, mstatus
	csrr	a5, mscratch
	call	machine_trap
