//! Privileged Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Privileged Insturctions.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrivOpcode {
    MRET,
    SRET,
    WFI,
    SFENCE_VMA,
}

impl Display for PrivOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PrivOpcode::SRET => write!(f, "sret"),
            PrivOpcode::MRET => write!(f, "mret"),
            PrivOpcode::WFI => write!(f, "wfi"),
            PrivOpcode::SFENCE_VMA => write!(f, "sfence.vma"),
        }
    }
}

impl Opcode for PrivOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            PrivOpcode::SRET | PrivOpcode::MRET | PrivOpcode::WFI => InstFormat::NoOperand,
            PrivOpcode::SFENCE_VMA => InstFormat::RFormat,
        }
    }
}
