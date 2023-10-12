//! raki
//!
//! `raki` is a RISC-V instruction decoder.
//!
//! # Usage
//! `decode` method is implemented for u16/u32.
//! ```
//! use raki::Isa;
//! use raki::decode::Decode;
//! use raki::instruction::Instruction;
//!
//! let inst: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
//! let inst: Instruction = match inst.decode(Isa::Rv32) {
//!     Ok(inst) => inst,
//!     Err(e) => panic!("decoding failed due to {e:?}"),
//! };
//! ```

pub mod decode;
pub mod instruction;

/// Target isa.
#[derive(Copy, Clone)]
pub enum Isa {
    Rv32,
    Rv64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn display_32bit_test() {
        use crate::decode::Decode;
        use crate::instruction::Instruction;
        use crate::Isa;

        let instructions: [u32; 3] = [
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            0b1101001100000011000011110010011,
            0b11100111001000010011010101111,
        ];

        for inst in &instructions {
            let inst: Instruction = match inst.decode(Isa::Rv32) {
                Ok(inst) => inst,
                Err(e) => panic!("decoding failed due to {e:?}"),
            };

            println!("{inst}");
        }
    }
}
