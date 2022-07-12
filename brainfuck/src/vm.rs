use std::io;
use std::io::{ErrorKind, Read};
use std::process::exit;

use crate::util::*;
use crate::Instruction::{ConJump, DecData, DecPtr, IncData, IncJump, IncPtr, Input, Output};
use crate::{Instruction, BUF_SIZE};

pub fn run(text: &Vec<Instruction>) {
    let mut data: Vec<u8> = vec![0; BUF_SIZE as usize];
    let mut dptr = 0usize;
    let mut iptr = 0usize;

    let mut buf = vec![0; 1];

    while iptr < text.len() {
        let op = text[iptr as usize];

        match op {
            IncPtr => {
                dptr = (dptr + 1) % BUF_SIZE;
                iptr += 1;
            }
            DecPtr => {
                dptr = (dptr - 1) % BUF_SIZE;
                iptr += 1;
            }
            IncData => {
                data[dptr] = data[dptr].wrapping_add(1);
                iptr += 1;
            }
            DecData => {
                data[dptr] = data[dptr].wrapping_sub(1);
                iptr += 1;
            }
            Input => {
                if let Err(e) = io::stdin().read_exact(&mut buf) {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => {
                            exit(0);
                        }
                        _ => {
                            panic!("Failed to read from stdin");
                        }
                    }
                }

                data[dptr] = buf[0];
                iptr += 1
            }
            Output => {
                print!("{}", data[dptr] as char);
                iptr += 1
            }
            ConJump(pos) => {
                if data[dptr] == 0 {
                    iptr = pos as usize;
                } else {
                    iptr += 1
                }
            }
            IncJump(pos) => {
                iptr = pos as usize;
            }
        }
    }
}

pub fn serialize(text: &Vec<Instruction>) -> Vec<u8> {
    let mut code = Vec::with_capacity(text.len());
    for &i in text {
        match i {
            IncPtr => code.push(0x01),
            DecPtr => code.push(0x02),
            IncData => code.push(0x03),
            DecData => code.push(0x04),
            Output => code.push(0x05),
            Input => code.push(0x06),
            IncJump(pos) => {
                code.push(0x07);
                code.push(pos);
            }
            ConJump(pos) => {
                code.push(0x08);
                code.push(pos);
            }
        }
    }

    u32_to_u8(&code)
}

pub fn deserialize(code: &Vec<u8>) -> Result<Vec<Instruction>, String> {
    let code = u8_to_u32(code);
    let mut text = Vec::with_capacity(code.len());

    let mut i = 0;
    while i < code.len() {
        match code[i] {
            0x01 => text.push(IncPtr),
            0x02 => text.push(DecPtr),
            0x03 => text.push(IncData),
            0x04 => text.push(DecData),
            0x05 => text.push(Output),
            0x06 => text.push(Input),
            0x07 => {
                if i + 1 < code.len() {
                    text.push(IncJump(code[i + 1]));
                    i += 1
                } else {
                    return Err("Invalid instruction".to_string());
                }
            }
            0x08 => {
                if i + 1 < code.len() {
                    text.push(ConJump(code[i + 1]));
                    i += 1
                } else {
                    return Err("Invalid instruction".to_string());
                }
            }
            _ => {
                return Err("Invalid instruction".to_string());
            }
        }

        i += 1
    }

    if is_valid(&text) {
        Ok(text)
    } else {
        Err("Invalid bytecode".to_string())
    }
}

fn is_valid(text: &Vec<Instruction>) -> bool {
    for &i in text {
        match i {
            ConJump(pos) => {
                if pos > text.len() as u32 {
                    return false;
                }
            }
            IncJump(pos) => {
                if pos > text.len() as u32 {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
