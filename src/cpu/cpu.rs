use super::super::interconnect;
use super::cp0::cp0;

const NUM_GPR: usize = 32;
const NUM_FPR: usize = 32;

#[derive(Debug)]
pub struct Cpu {
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_FPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, // TODO: Enum type

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0,

    interconnect: interconnect::Interconnect,
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_FPR],

            reg_pc: 0,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: cp0::Cp0::default(),

            interconnect: interconnect,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();

        self.reg_pc = 0xffff_ffff_bfc0_0000; // TODO: Move to const
    }

    // TODO: Different interface
    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instruction = self.read_word(self.reg_pc);

        let opcode = (instruction >> 26) & 0b111111;
        let rs = (instruction >> 21) & 0b11111;
        let rt = (instruction >> 16) & 0b11111;
        let imm = instruction & 0xffff;

        match opcode {
            0b001101 => {
                // ORI
                let result = self.read_reg_gpr(rs as usize) | (imm as u64);
                self.write_reg_gpr(rt as usize, result)
            }
            0b001111 => {
                // LUI
                // TODO: Sign extend for upper 32 bits
                let value = ((imm << 16) as i32) as u64;
                self.write_reg_gpr(rt as usize, value);
            }
            0b010000 => {
                // MTC0
                let rd = instruction >> 11 & 0b11111;
                let data = self.read_reg_gpr(rt as usize);
                self.cp0.write_reg(rd, data);
            }
            0b100011 => {
                // LW
                // TODO: Sign extend for upper 32 bits
                let base = rs;
                let offset = imm;
                let sign_extended_offset = (offset as i16) as u64;
                let virtual_addr = sign_extended_offset + self.read_reg_gpr(base as usize);
                let mem = (self.read_word(virtual_addr) as i32) as u64;
                self.write_reg_gpr(rt as usize, mem);
            }
            _ => panic!("Unrecognized instruction: {:#x}", instruction),
        }

        self.reg_pc += 4;
    }

    fn read_word(&self, virtual_addr: u64) -> u32 {
        let physical_addr = self.virtual_addr_to_physical_addr(virtual_addr);
        // TODO: Check endianness
        self.interconnect.read_word(physical_addr as u32)
    }

    fn virtual_addr_to_physical_addr(&self, virtual_addr: u64) -> u64 {
        // VR4300 user manual page 131. See Table 5-3
        let addr_bit_values = (virtual_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 {
            // kseg1
            virtual_addr - 0xffff_ffff_a000_0000
        } else {
            // TODO;
            panic!("Unrecognized virtual address: {:#x}", virtual_addr)
        }
    }

    fn write_reg_gpr(&mut self, index: usize, data: u64) {
        if index != 0 {
            self.reg_gpr[index] = data;
        }
    }

    fn read_reg_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_gpr[index],
        }
    }
}
