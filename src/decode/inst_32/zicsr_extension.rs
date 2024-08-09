use super::super::{DecodeUtil, DecodingError};
use crate::instruction::zicsr_extension::ZicsrOpcode;

pub fn parse_opcode(inst: u32) -> Result<ZicsrOpcode, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b111_0011 => match funct3 {
            0b001 => Ok(ZicsrOpcode::CSRRW),
            0b010 => Ok(ZicsrOpcode::CSRRS),
            0b011 => Ok(ZicsrOpcode::CSRRC),
            0b101 => Ok(ZicsrOpcode::CSRRWI),
            0b110 => Ok(ZicsrOpcode::CSRRSI),
            0b111 => Ok(ZicsrOpcode::CSRRCI),
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &ZicsrOpcode) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        ZicsrOpcode::CSRRW
        | ZicsrOpcode::CSRRS
        | ZicsrOpcode::CSRRC
        | ZicsrOpcode::CSRRWI
        | ZicsrOpcode::CSRRSI
        | ZicsrOpcode::CSRRCI => Ok(Some(rd)),
    }
}

pub fn parse_rs1(inst: u32, opkind: &ZicsrOpcode) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        ZicsrOpcode::CSRRW | ZicsrOpcode::CSRRS | ZicsrOpcode::CSRRC => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &ZicsrOpcode) -> Result<Option<usize>, DecodingError> {
    let csr: usize = inst.slice(31, 20) as usize;

    match opkind {
        ZicsrOpcode::CSRRW
        | ZicsrOpcode::CSRRS
        | ZicsrOpcode::CSRRC
        | ZicsrOpcode::CSRRWI
        | ZicsrOpcode::CSRRSI
        | ZicsrOpcode::CSRRCI => Ok(Some(csr)),
    }
}

pub fn parse_imm(inst: u32, opkind: &ZicsrOpcode) -> Result<Option<i32>, DecodingError> {
    let uimm: i32 = inst.slice(19, 15) as i32;
    match opkind {
        ZicsrOpcode::CSRRWI | ZicsrOpcode::CSRRSI | ZicsrOpcode::CSRRCI => Ok(Some(uimm)),
        _ => Ok(None),
    }
}
