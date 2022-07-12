extern crate core;

use std::{fs, io};

use argparse::{ArgumentParser, Store};

fn main() -> Result<(), io::Error> {
    let mut mode = String::new();
    let mut file_in = String::new();
    let mut file_out = String::new();
    let mut platform = String::from("bfvm");
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut mode)
            .add_argument("mode", Store, "build | run");
        ap.refer(&mut file_in)
            .add_argument("input", Store, "Input file");
        ap.refer(&mut file_out)
            .add_option(&["-o", "--output"], Store, "Output file");
        ap.refer(&mut platform)
            .add_option(&["--platform"], Store, "Target platform");

        ap.parse_args_or_exit();
    }

    match &mode[..] {
        "build" => {
            let src = fs::read_to_string(file_in)?;
            let text = brainfuck::frontend::compile(&src)
                .expect("Invalid source code");

            match &platform[..] {
                "bfvm" => {
                    if file_out.is_empty() {
                        file_out = String::from("out.bfvm")
                    }

                    fs::write(file_out, brainfuck::vm::serialize(&text))
                        .expect("Failed to write bytecode");
                }
                "x86_64-linux" => {
                    if file_out.is_empty() {
                        file_out = String::from("out.x86_64")
                    }

                    brainfuck::backend::x86_64::linux::compile(&text, &file_out)?;
                }
                _ => {
                    eprintln!("Invalid platform");
                }
            }
        }
        "run" => {
            let src = fs::read_to_string(file_in)?;
            let text = brainfuck::frontend::compile(&src)
                .expect("Invalid source code");

            brainfuck::vm::run(&text);
        }
        _ => {
            eprintln!("Invalid commandline");
        }
    }

    Ok(())
}
