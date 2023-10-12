//! raki
//!
//! `raki` is a RISC-V instruction decoder written in Rust.
//!
//! - Both 32/64bit support.
//! - Support `rv32/64imac`, `Zicsr`, `Zifencei` extensions.
//! - Implement Display trait for formatting.
//!
//! # Usage
//! Call the `decode` as u16/u32 method.
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
//! println!("{inst}");
//! // --output--
//! // addi t0, t0, -276
//! ```

pub mod decode;
pub mod instruction;

/// Target isa.
#[derive(Copy, Clone)]
pub enum Isa {
    /// 32 bit architecture.
    Rv32,
    /// 64 bit architecture.
    Rv64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn display_32bit_test() {
        use crate::decode::Decode;
        use crate::instruction::Instruction;
        use crate::Isa;

        let instructions: [u32; 8] = [
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            0b110_1001_1000_0001_1000_0111_1001_0011,
            0b1_1100_1110_0100_0010_0110_1010_1111,
            0b1111_0101_0000_0000_0000_0000_1111,
            0b1000_1111_0100_0010_0111_1010_1111,
            0b111_0011,
            0b110_0101_1111_0000_0011_0000_1110_1111,
            0b110_1001_0111,
        ];

        for inst in &instructions {
            let inst: Instruction = match inst.decode(Isa::Rv32) {
                Ok(inst) => inst,
                Err(e) => panic!("decoding failed due to {e:?}"),
            };

            println!("{inst}");
        }
    }

    #[test]
    fn display_16bit_test() {
        use crate::decode::Decode;
        use crate::instruction::Instruction;
        use crate::Isa;

        let instructions: [u16; 8] = [
            0b1110010000100110,
            0b110110000000100,
            0b1110010100010001,
            0b1000010100100110,
            0b1011011111110101,
            0b110000,
            0b1000000010000010,
            0b11011111111101,
        ];

        for inst in &instructions {
            let inst: Instruction = match inst.decode(Isa::Rv64) {
                Ok(inst) => inst,
                Err(e) => panic!("decoding failed due to {e:?}"),
            };

            println!("{inst}");
        }
    }
}