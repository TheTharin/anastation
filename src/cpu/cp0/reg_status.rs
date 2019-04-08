#[derive(Debug, Default)]
pub struct RegStatus {
    // CU
    coprocessor_usability: [bool; 4],

    // RP
    low_power: bool,

    // FR
    additional_fp_regs: bool,

    // RE
    reverse_endian: bool,

    // DS
    diagnostic_status: SelfDiagnosticStatusField,

    // IM(7:0)
    interrupt_mask: InterruptMask,

    // KX
    kernel_mode_64bit_addressing: bool,

    // SX
    supervisor_mode_64bit_addressing: bool,

    // UX
    user_mode_64bit_addressing: bool,

    // KSU
    mode: Mode,

    // ERl
    error_level: bool,

    // EXL
    exception_level: bool,

    // IE
    global_interrupts_enable: bool,
}

impl From<u32> for RegStatus {
    fn from(data: u32) -> Self {
        RegStatus {
            coprocessor_usability: [
                (data & (1 << 31)) != 0,
                (data & (1 << 30)) != 0,
                (data & (1 << 29)) != 0,
                (data & (1 << 28)) != 0,
            ],

            low_power: (data & (1 << 27)) != 0,
            additional_fp_regs: (data & (1 << 26)) != 0,
            reverse_endian: (data & (1 << 25)) != 0,

            diagnostic_status: data.into(),
            interrupt_mask: data.into(),

            kernel_mode_64bit_addressing: (data & (1 << 7)) != 0,
            supervisor_mode_64bit_addressing: (data & (1 << 6)) != 0,
            user_mode_64bit_addressing: (data & (1 << 5)) != 0,

            mode: data.into(),

            error_level: (data & (1 << 2)) != 0,
            exception_level: (data & (1 << 1)) != 0,
            global_interrupts_enable: (data & (1 << 0)) != 0,
        }
    }
}

#[derive(Debug, Default)]
struct SelfDiagnosticStatusField {
    // ITS
    instruction_trace_support: bool,

    // BEV
    // TODO: Better name?
    tlb_and_general_exception_vectors_location: TLBGeneralExceptionVectorsLocation,

    // TS
    tlb_shutdown: bool,

    // SR
    soft_reset_or_nmi_occurred: bool,

    // CH
    condition_bit: bool,
}

impl From<u32> for SelfDiagnosticStatusField {
    fn from(data: u32) -> Self {
        SelfDiagnosticStatusField {
            instruction_trace_support: (data & (1 << 24)) != 0,

            tlb_and_general_exception_vectors_location: data.into(),

            tlb_shutdown: (data & (1 << 21)) != 0,

            soft_reset_or_nmi_occurred: (data & (1 << 20)) != 0,

            condition_bit: (data & (1 << 18)) != 0,
        }
    }
}

// TODO: Better name?
#[derive(Debug)]
enum TLBGeneralExceptionVectorsLocation {
    Normal,
    Bootstrap,
}

impl Default for TLBGeneralExceptionVectorsLocation {
    fn default() -> Self {
        TLBGeneralExceptionVectorsLocation::Normal
    }
}

impl From<u32> for TLBGeneralExceptionVectorsLocation {
    fn from(data: u32) -> Self {
        match (data >> 22) & 0b1 {
            0 => TLBGeneralExceptionVectorsLocation::Normal,
            1 => TLBGeneralExceptionVectorsLocation::Bootstrap,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct InterruptMask {
    // IM(7)
    timer_interrupt: bool,

    // IM(6:2)
    external_interrupt_write_req: [bool; 5],

    // IM(1:0)
    software_interrupt_causer_reg: [bool; 2],
}

impl From<u32> for InterruptMask {
    fn from(data: u32) -> Self {
        InterruptMask {
            timer_interrupt: (data & (1 << 15)) != 0,
            external_interrupt_write_req: [
                (data & (1 << 10)) != 0,
                (data & (1 << 11)) != 0,
                (data & (1 << 12)) != 0,
                (data & (1 << 13)) != 0,
                (data & (1 << 14)) != 0,
            ],
            software_interrupt_causer_reg: [(data & (1 << 8)) != 0, (data & (1 << 9)) != 0],
        }
    }
}

#[derive(Debug)]
enum Mode {
    Kernel,
    Supervisor,
    User,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Kernel
    }
}

impl From<u32> for Mode {
    fn from(data: u32) -> Self {
        match (data >> 3) & 0b11 {
            0b00 => Mode::Kernel,
            0b01 => Mode::Supervisor,
            0b10 => Mode::User,
            _ => panic!("Invalid KSU bits: {:#b}", data),
        }
    }
}
