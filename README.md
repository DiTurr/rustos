<p align="center">
  <img src="doc/logo.png"width="25%" />
</p>

[Table of Contents](#table-of-contents)
- [1. Target HW:](#1-target-hw)
- [2. RISC-V CPU registers:](#2-risc-v-cpu-registers)
  - [2.1. Interger registers:](#21-interger-registers)
  - [2.2. Floating point registes:](#22-floating-point-registes)
  - [2.3. Control and Status Registers (CSRs):](#23-control-and-status-registers-csrs)
    - [2.3.1. Machine-Mode CSRs](#231-machine-mode-csrs)
    - [2.3.2. Supervisor-Mode CSRs](#232-supervisor-mode-csrs)
    - [2.3.3. Universal/Performance CSRs](#233-universalperformance-csrs)
- [3. Memory map:](#3-memory-map)
  - [3.1. QEMU memory map:](#31-qemu-memory-map)
- [4. RISC-V Machine Trap Codes (`mcause` values)](#4-risc-v-machine-trap-codes-mcause-values)
  - [4.1. Machine-Level Interrupts (MSB = 1, `mcause` ≥ 0x80000000)](#41-machine-level-interrupts-msb--1-mcause--0x80000000)
  - [4.2. Machine-Level Exceptions (MSB = 0, `mcause` \< 0x80000000)](#42-machine-level-exceptions-msb--0-mcause--0x80000000)
- [5. Memory Management:](#5-memory-management)

# 1. Target HW:
Target is RISC-V RV64IMAFDC (riscv64gc-unknown-none-elf):
 - RV64: 64-bit base integer instruction set (registers and addresses are 64-bit).
 - I: Base Integer instructions (required).
 - M: Integer Multiplication and Division.
 - A: Atomic instructions (for concurrency).
 - F: Single-precision floating-point instructions.
 - D: Double-precision floating-point instructions.
 - C: Compressed instructions (16-bit versions of common instructions to save space).

So RV64IMAFDC is a 64-bit RISC-V core with support for integer arithmetic, atomic operations, and floating-point math (both single and double precision), plus compressed instructions.

# 2. RISC-V CPU registers:
+---------------------+--------------------+-----------------------------+
| Category            | Names              | Usage / Notes               |
+---------------------+--------------------+-----------------------------+
| General-Purpose     | x0–x31             | Core computation and control|
| (GPR)               | zero, ra, sp, ...  | ABI-compliant, 32 total     |
+---------------------+--------------------+-----------------------------+
| Floating-Point      | f0–f31             | Optional, for F/D extensions|
| (FPR)               | ft0, fa0, fs0, ... | Used for math, SIMD, etc.   |
+---------------------+--------------------+-----------------------------+
| Control & Status    | mstatus, sepc, ... | CSRs accessed via `csrr/w`  |
| (CSR)               | 0x000–0xFFF        | Trap handling, paging, etc. |
+---------------------+--------------------+-----------------------------+
| Memory-Mapped I/O   | mtime, mtimecmp,   | Timer, UART, PLIC, etc.     |
| (Platform-specific) | UART registers     | Based on hardware platform  |
+---------------------+--------------------+-----------------------------+

## 2.1. Interger registers:
| Register | ABI Name | Description          |
|----------|----------|----------------------|
| x0       | zero     | Constant zero        |
| x1       | ra       | Return address       |
| x2       | sp       | Stack pointer        |
| x3       | gp       | Global pointer       |
| x4       | tp       | Thread pointer       |
| x5       | t0       | Temporary            |
| x6       | t1       | Temporary            |
| x7       | t2       | Temporary            |
| x8       | s0/fp    | Saved/frame pointer  |
| x9       | s1       | Saved register       |
| x10      | a0       | Function arg/return  |
| x11      | a1       | Function arg/return  |
| x12      | a2       | Function argument    |
| x13      | a3       | Function argument    |
| x14      | a4       | Function argument    |
| x15      | a5       | Function argument    |
| x16      | a6       | Function argument    |
| x17      | a7       | Function argument    |
| x18      | s2       | Saved register       |
| x19      | s3       | Saved register       |
| x20      | s4       | Saved register       |
| x21      | s5       | Saved register       |
| x22      | s6       | Saved register       |
| x23      | s7       | Saved register       |
| x24      | s8       | Saved register       |
| x25      | s9       | Saved register       |
| x26      | s10      | Saved register       |
| x27      | s11      | Saved register       |
| x28      | t3       | Temporary            |
| x29      | t4       | Temporary            |
| x30      | t5       | Temporary            |
| x31      | t6       | Temporary            |

## 2.2. Floating point registes:
| Register | ABI Name | Description          |
|----------|----------|----------------------|
| f0       | ft0      | FP temporary         |
| f1       | ft1      | FP temporary         |
| f2       | ft2      | FP temporary         |
| f3       | ft3      | FP temporary         |
| f4       | ft4      | FP temporary         |
| f5       | ft5      | FP temporary         |
| f6       | ft6      | FP temporary         |
| f7       | ft7      | FP temporary         |
| f8       | fs0      | FP saved register    |
| f9       | fs1      | FP saved register    |
| f10      | fa0      | FP arg/return        |
| f11      | fa1      | FP arg/return        |
| f12      | fa2      | FP argument          |
| f13      | fa3      | FP argument          |
| f14      | fa4      | FP argument          |
| f15      | fa5      | FP argument          |
| f16      | fa6      | FP argument          |
| f17      | fa7      | FP argument          |
| f18      | fs2      | FP saved register    |
| f19      | fs3      | FP saved register    |
| f20      | fs4      | FP saved register    |
| f21      | fs5      | FP saved register    |
| f22      | fs6      | FP saved register    |
| f23      | fs7      | FP saved register    |
| f24      | fs8      | FP saved register    |
| f25      | fs9      | FP saved register    |
| f26      | fs10     | FP saved register    |
| f27      | fs11     | FP saved register    |
| f28      | ft8      | FP temporary         |
| f29      | ft9      | FP temporary         |
| f30      | ft10     | FP temporary         |
| f31      | ft11     | FP temporary         |

## 2.3. Control and Status Registers (CSRs):
Access control:
- **Machine-mode CSRs** are only accessible in `M` mode.
- **Supervisor-mode CSRs** require entering `S` mode (common for kernel/OS).
- **User-level code** cannot access `m*` or `s*` CSRs unless delegated.
- `mcounteren` determines whether counters are visible in lower modes.

### 2.3.1. Machine-Mode CSRs
| CSR Name     | Address   | Description                                                    |
|--------------|-----------|----------------------------------------------------------------|
| `mstatus`    | 0x300     | Machine status register (global interrupt/privilege flags)     |
| `misa`       | 0x301     | ISA and supported extensions (e.g., RV64IMAFDC)                |
| `medeleg`    | 0x302     | Machine exception delegation to S-mode                         |
| `mideleg`    | 0x303     | Machine interrupt delegation to S-mode                         |
| `mie`        | 0x304     | Machine interrupt-enable register                              |
| `mtvec`      | 0x305     | Machine trap-handler base address                              |
| `mcounteren` | 0x306     | Enables counters for S/U mode                                  |
| `mscratch`   | 0x340     | Scratch register for M-mode context switches                   |
| `mepc`       | 0x341     | Machine exception program counter                              |
| `mcause`     | 0x342     | Machine trap cause (interrupt or exception ID)                 |
| `mtval`      | 0x343     | Machine trap value (e.g., bad address for page faults)         |
| `mip`        | 0x344     | Machine interrupt pending register                             |
| `mcycle`     | 0xB00     | Machine cycle counter                                          |
| `minstret`   | 0xB02     | Machine instructions-retired counter                           |
| `mhartid`    | 0xF14     | Hardware thread ID (hart ID)                                   |

### 2.3.2. Supervisor-Mode CSRs
| CSR Name      | Address   | Description                                                    |
|---------------|-----------|----------------------------------------------------------------|
| `sstatus`     | 0x100     | Supervisor status register                                     |
| `sie`         | 0x104     | Supervisor interrupt-enable register                           |
| `stvec`       | 0x105     | Supervisor trap-vector base address                            |
| `sscratch`    | 0x140     | Supervisor scratch register                                    |
| `sepc`        | 0x141     | Supervisor exception program counter                           |
| `scause`      | 0x142     | Supervisor trap cause                                          |
| `stval`       | 0x143     | Supervisor trap value                                          |
| `sip`         | 0x144     | Supervisor interrupt pending register                          |
| `satp`        | 0x180     | Supervisor address translation and protection (page table root)|

### 2.3.3. Universal/Performance CSRs
| CSR Name      | Address   | Description                                                             |
|---------------|-----------|-------------------------------------------------------------------------|
| `cycle`       | 0xC00     | Cycle counter (readable from all modes if enabled)                      |
| `time`        | 0xC01     | Timer value (`mtime` view) — read-only CSR (number of ticks since boot) |
| `instret`     | 0xC02     | Instructions retired                                                    |

`cycle`, `time`, and `instret` access depends on `mcounteren` configuration in lower modes.

# 3. Memory map:
## 3.1. QEMU memory map:
RISC-V memory map from (https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c):

|  Entry                | Start      | Size                               |
|-----------------------|------------|------------------------------------|
|  [VIRT_DEBUG]         |        0x0 |  0x100                             |
|  [VIRT_MROM]          |     0x1000 |  0xf000                            |
|  [VIRT_TEST]          |   0x100000 |  0x1000                            |
|  [VIRT_RTC]           |   0x101000 |  0x1000                            |
|  [VIRT_CLINT]         |  0x2000000 |  0x10000                           |
|  [VIRT_ACLINT_SSWI]   |  0x2F00000 |  0x4000                            |
|  [VIRT_PCIE_PIO]      |  0x3000000 |  0x10000                           |
|  [VIRT_IOMMU_SYS]     |  0x3010000 |  0x1000                            |
|  [VIRT_PLATFORM_BUS]  |  0x4000000 |  0x2000000                         |
|  [VIRT_PLIC]          |  0xc000000 |  VIRT_PLIC_SIZE(VIRT_CPUS_MAX * 2) |
|  [VIRT_APLIC_M]       |  0xc000000 |  APLIC_SIZE(VIRT_CPUS_MAX)         |
|  [VIRT_APLIC_S]       |  0xd000000 |  APLIC_SIZE(VIRT_CPUS_MAX)         |
|  [VIRT_UART0]         | 0x10000000 |  0x100                             |
|  [VIRT_VIRTIO]        | 0x10001000 |  0x1000                            |
|  [VIRT_FW_CFG]        | 0x10100000 |  0x18                              |
|  [VIRT_FLASH]         | 0x20000000 |  0x4000000                         |
|  [VIRT_IMSIC_M]       | 0x24000000 |  VIRT_IMSIC_MAX_SIZE               |
|  [VIRT_IMSIC_S]       | 0x28000000 |  VIRT_IMSIC_MAX_SIZE               |
|  [VIRT_PCIE_ECAM]     | 0x30000000 |  0x10000000                        |
|  [VIRT_PCIE_MMIO]     | 0x40000000 |  0x40000000                        |
|  [VIRT_DRAM]          | 0x80000000 |  0x0                               |

# 4. RISC-V Machine Trap Codes (`mcause` values)
## 4.1. Machine-Level Interrupts (MSB = 1, `mcause` ≥ 0x80000000)
| Code (dec) | `mcause` (hex) | Description                                   |
|------------|----------------|-----------------------------------------------|
| 3          | 0x80000003     | Machine software interrupt                    |
| 7          | 0x80000007     | Machine timer interrupt                       |
| 11         | 0x8000000B     | Machine external interrupt                    |
| 13         | 0x8000000D     | Platform-specific counter interrupt (optional)|

## 4.2. Machine-Level Exceptions (MSB = 0, `mcause` < 0x80000000)
| Code (dec) | `mcause` (hex) | Description                        |
|------------|----------------|------------------------------------|
| 0          | 0x00000000     | Instruction address misaligned     |
| 1          | 0x00000001     | Instruction access fault           |
| 2          | 0x00000002     | Illegal instruction                |
| 3          | 0x00000003     | Breakpoint                         |
| 4          | 0x00000004     | Load address misaligned            |
| 5          | 0x00000005     | Load access fault                  |
| 6          | 0x00000006     | Store/AMO address misaligned       |
| 7          | 0x00000007     | Store/AMO access fault             |
| 8          | 0x00000008     | Environment call from U-mode       |
| 9          | 0x00000009     | Environment call from S-mode       |
| 11         | 0x0000000B     | Environment call from M-mode       |
| 12         | 0x0000000C     | Instruction page fault             |
| 13         | 0x0000000D     | Load page fault                    |
| 15         | 0x0000000F     | Store/AMO page fault               |
| 18         | 0x00000012     | Software check (optional)          |
| 19         | 0x00000013     | Hardware error (optional)          |

# 5. Memory Management:



```bash
FUNCTION="kmain"
riscv64-unknown-elf-objdump -d ./target/elf/rustos.elf --disassemble=${FUNCTION}
```

