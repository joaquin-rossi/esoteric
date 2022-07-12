use crate::Instruction::{ConJump, DecData, DecPtr, IncData, IncJump, IncPtr, Input, Output};
use std::collections::HashMap;

pub mod backend;
pub mod frontend;
pub mod vm;

mod util;

pub const BUF_SIZE: usize = 30000;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    IncPtr,
    DecPtr,
    IncData,
    DecData,
    Input,
    Output,
    IncJump(u32),
    ConJump(u32),
}
