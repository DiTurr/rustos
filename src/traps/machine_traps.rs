//! ---------------------------------------------------------------------------
//! File       : machine_traps.rs
//! Module     : traps::machine_traps
//! Author     : DiTurr
//! Description:
//! This module defines the `machine_trap` handler, which is invoked when a trap
//! (exception or interrupt) occurs in RISC-V Machine mode.
//! ---------------------------------------------------------------------------

use crate::log_info;
use crate::traps::traps::Trap;

/// Trap handler for exceptions and interrupts occurring in Machine mode.
/// This function is called directly from the trap vector (typically via `mtvec`)
/// when an exception or interrupt is taken while the CPU is in **Machine mode**.
///
/// # Context
/// - Runs in **Machine mode (M-mode)**
/// - **Interrupts and MMU are disabled**
/// - Full privileged access to hardware is available
///
/// # Responsibilities
/// - Print diagnostic information (register values at time of trap)
/// - Provide a single point for debugging trap causes
/// - Halt the system to avoid undefined behavior in the absence of recovery
///
/// # Future Extensions
/// - Decode and handle specific trap causes (`mcause`)
/// - Delegate to Supervisor mode (`sret`) if MMU and traps are initialized
/// - Implement per-hart context management via `mscratch`
///
/// # Parameters:
/// - `mepc`: Machine Exception Program Counter (where the exception occurred)
/// - `mtval`: Trap value (e.g., faulting address)
/// - `mcause`: Trap cause (interrupt or exception ID)
/// - `mhartid`: Current hardware thread ID
/// - `mstatus`: Machine status register at time of trap
/// - `mscratch`: Value of the scratch register (can be used for context switch)
///
/// # Safety:
/// - Marked `unsafe` because it's called directly by the trap vector and must adhere
///   to the ABI and calling conventions of the hardware.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn machine_trap(
    mepc: usize,
    mtval: usize,
    mcause: usize,
    mhartid: usize,
    mstatus: usize,
    mscratch: usize,
) -> ! {
    // Log full trap state for debugging purposes
    log_info!(
        "Machine trap. \
        MEPC: 0x{:08x} - \
        MTVAL: 0x{:08x} - \
        MCAUSE: 0x{:08x} - \
        MHARTID: 0x{:08x} - \
        MSTATUS: 0x{:08x} - \
        MSCRATCH: 0x{:08x}",
        mepc, mtval, mcause, mhartid, mstatus, mscratch
    );
    // Select reaction depending on trap type
    match mcause {
        // === Exception Codes (synchronous traps) ===
        val if val == Trap::InstructionMisaligned as usize => {
            panic!("Unhandled Instruction Misaligned Trap.");
        }
        val if val == Trap::InstructionAccessFault as usize => {
            panic!("Unhandled Instruction Access Fault Trap.");
        }
        val if val == Trap::IllegalInstruction as usize => {
            panic!("Unhandled Illegal Instruction Trap.");
        }
        val if val == Trap::Breakpoint as usize => {
            panic!("Unhandled Breakpoint Trap.");
        }
        val if val == Trap::LoadMisaligned as usize => {
            panic!("Unhandled Load Misaligned Trap.");
        }
        val if val == Trap::LoadAccessFault as usize => {
            panic!("Unhandled Load Access Fault Trap.");
        }
        val if val == Trap::StoreMisaligned as usize => {
            panic!("Unhandled Store Misaligned Trap.");
        }
        val if val == Trap::StoreAccessFault as usize => {
            panic!("Unhandled Store Access Fault Trap.");
        }
        val if val == Trap::UserEnvCall as usize => {
            panic!("Unhandled User Environment Call Trap.");
        }
        val if val == Trap::SupervisorEnvCall as usize => {
            panic!("Unhandled Supervisor Environment Call Trap.");
        }
        val if val == Trap::MachineEnvCall as usize => {
            panic!("Unhandled Machine Environment Call Trap.");
        }
        val if val == Trap::InstructionPageFault as usize => {
            panic!("Unhandled Instruction Page Fault Trap.");
        }
        val if val == Trap::LoadPageFault as usize => {
            panic!("Unhandled Load Page Fault Trap.");
        }
        val if val == Trap::StorePageFault as usize => {
            panic!("Unhandled Store Page Fault Trap.");
        }
        // === Interrupt Codes (asynchronous traps) ===
        val if val == Trap::SupervisorSoftInterrupt as usize => {
            panic!("Unhandled Supervisor Software Interrupt.");
        }
        val if val == Trap::MachineSoftInterrupt as usize => {
            panic!("Unhandled Machine Software Interrupt.");
        }
        val if val == Trap::SupervisorTimerInterrupt as usize => {
            panic!("Unhandled Supervisor Timer Interrupt.");
        }
        val if val == Trap::MachineTimerInterrupt as usize => {
            panic!("Unhandled Machine Timer Interrupt.");
        }
        val if val == Trap::SupervisorExternalInterrupt as usize => {
            panic!("Unhandled Supervisor External Interrupt.");
        }
        val if val == Trap::MachineExternalInterrupt as usize => {
            panic!("Unhandled Machine External Interrupt.");
        }
        // === Unknown or unhandled trap code ===
        _ => {
            panic!("Unhandled unknown machine trap: 0x{:x}", mcause);
        }
    }
}
