/// Represents all RISC-V trap causes, including exceptions and interrupts.
///
/// The upper bit (bit XLEN-1) of mcause/scause distinguishes
/// between interrupts and exceptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Trap {
    // === Exception Codes (MSB = 0) ===
    InstructionMisaligned         = 0,
    InstructionAccessFault        = 1,
    IllegalInstruction            = 2,
    Breakpoint                    = 3,
    LoadMisaligned                = 4,
    LoadAccessFault               = 5,
    StoreMisaligned               = 6,
    StoreAccessFault              = 7,
    UserEnvCall                   = 8,
    SupervisorEnvCall             = 9,
    MachineEnvCall                = 11,
    InstructionPageFault          = 12,
    LoadPageFault                 = 13,
    StorePageFault                = 15,
    // === Interrupt Codes (MSB = 1, OR-ed with LSB) ===
    SupervisorSoftInterrupt       = 1 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
    MachineSoftInterrupt          = 3 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
    SupervisorTimerInterrupt      = 5 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
    MachineTimerInterrupt         = 7 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
    SupervisorExternalInterrupt   = 9 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
    MachineExternalInterrupt      = 11 | (1 << (core::mem::size_of::<usize>() * 8 - 1)),
}
