use std::{fs, io};
use std::process::Command;
use crate::Instruction;

pub fn compile(text: &Vec<Instruction>, file_out: &String) -> io::Result<()> {
    let mut code = "\
BITS 64

segment .bss
buffer: resb 30000

segment .text
input:
    mov rsi, rdi
    mov edx, 1
    mov edi, 0
    mov rax, 0
    syscall
    ret

output:
    mov rsi, rdi
    mov edx, 1
    mov edi, 1
    mov rax, 1
    syscall
    ret

global _start
_start:
    mov QWORD rbp, buffer
".to_string();

    for (iptr, &i) in text.iter().enumerate() {
        code.push_str(&format!("addr_{}:\n", iptr));

        match i {
            Instruction::IncPtr => {
                code.push_str("    add QWORD rbp, 1\n");
            }
            Instruction::DecPtr => {
                code.push_str("    sub QWORD rbp, 1\n");
            }
            Instruction::IncData => {
                code.push_str("    mov   rax, QWORD rbp\n");
                code.push_str("    movzx eax, BYTE [rax]\n");
                code.push_str("    add   eax, 1\n");
                code.push_str("    mov   edx, eax\n");
                code.push_str("    mov   rax, QWORD rbp\n");
                code.push_str("    mov   BYTE [rax], dl\n");
            }
            Instruction::DecData => {
                code.push_str("    mov   rax, QWORD rbp\n");
                code.push_str("    movzx eax, BYTE [rax]\n");
                code.push_str("    sub   eax, 1\n");
                code.push_str("    mov   edx, eax\n");
                code.push_str("    mov   rax, QWORD rbp\n");
                code.push_str("    mov   BYTE [rax], dl\n");
            }
            Instruction::Input => {
                code.push_str("    mov  rdi, rbp\n");
                code.push_str("    call input\n");
            }
            Instruction::Output => {
                code.push_str("    mov  rdi, rbp\n");
                code.push_str("    call output\n");
            }
            Instruction::IncJump(pos) => {
                code.push_str(&format!("    jmp addr_{}\n", pos))
            }
            Instruction::ConJump(pos) => {
                code.push_str("    mov rax, QWORD rbp\n");
                code.push_str("    movzx eax, BYTE [rax]\n");
                code.push_str("    cmp eax, 0\n");
                code.push_str(&format!("    jle addr_{}\n", pos));
            }
        }
    }

    let str = format!("\
addr_{}:
    mov rax, 60
    mov rdi, 0
    syscall
", text.len());
    code.push_str(&str);

    fs::write(format!("{}.asm", file_out), code)
        .expect("Failed to write code");

    Command::new("nasm")
        .args(["-felf64", &format!("{}.asm", file_out)])
        .spawn()?
        .wait()?;

    Command::new("ld")
        .args(["-o", file_out, &format!("{}.o", file_out)])
        .spawn()?
        .wait()?;

    fs::remove_file(format!("{}.asm", file_out))?;
    fs::remove_file(format!("{}.o", file_out))?;

    Ok(())
}
