################
## CONFIGURATION
################
TYPE:=debug
TARGET_DIR:=./target
OBJ_DIR:=${TARGET_DIR}/obj
ELF_DIR:=${TARGET_DIR}/elf
ELF_FILE:=${ELF_DIR}/rustos.elf

################
## COMPILE
################
CC=riscv64-unknown-elf-gcc
CFLAGS=-Wall -Wextra -Wpedantic -Wextra -O0
CFLAGS+=-ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv64gc -mabi=lp64d
ASM_DIR:=./src/asm
ASM_FILES:=$(wildcard $(ASM_DIR)/*.S)
ASM_OBJS:=$(patsubst $(ASM_DIR)/%.S,$(OBJ_DIR)/%.o,$(ASM_FILES))

################
## LINK
################
LD:=riscv64-unknown-elf-gcc
LDFLAGS:=-static -nostdlib
LDSCRIPT:=lds/virt.lds

################
## QEMU
################
QEMU:=qemu-system-riscv64
MACH:=virt
CPU:=rv64
MEM:=128M

################
# obj directory creation
################
obj_dir/:
	@mkdir -p ${OBJ_DIR}

################
# elf directory creation
################
elf_dir/:
	@mkdir -p ${ELF_DIR}

################
# Compile assembly (.S) code
################
asm: $(ASM_OBJS)

################
# Pattern rule to compile .S -> .o
################
$(OBJ_DIR)/%.o: $(ASM_DIR)/%.S | obj_dir/
	${CC} $(CFLAGS) -c -o $@ $<

################
# Compile Rust code
################
rust: | obj_dir/
	CARGO_TARGET_DIR=${TARGET_DIR} RUSTFLAGS="--emit=obj" cargo +nightly build -Z build-std=core,compiler_builtins
	@cp ${TARGET_DIR}/riscv64gc-unknown-none-elf/${TYPE}/deps/*.o ${OBJ_DIR}
	@rm -r ${TARGET_DIR}/${TYPE}

################
# Compile and link
################
all: asm rust | elf_dir/
	@OBJS="$(wildcard ${OBJ_DIR}/*.o)"; \
	${LD} ${LDFLAGS} $$OBJS -T ${LDSCRIPT} -o ${ELF_FILE}

run:
	$(QEMU) \
	-machine $(MACH) \
	-cpu $(CPU) \
	-smp 1 \
	-m $(MEM) \
	-nographic \
	-serial mon:stdio \
	-bios none \
	-kernel $(ELF_FILE)
# -d in_asm

################
# Clean build artifacts
################
.PHONY: clean
clean:
	cargo clean
