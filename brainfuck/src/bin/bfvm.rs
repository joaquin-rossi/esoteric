use std::{fs, io};

use argparse::{ArgumentParser, Store};

fn main() -> Result<(), io::Error> {
    let mut file_in = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_in)
            .add_argument("input", Store, "Input file");

        ap.parse_args_or_exit();
    }

    let code = fs::read(file_in)?;
    let text = brainfuck::vm::deserialize(&code).expect("Invalid bytecode");

    brainfuck::vm::run(&text);

    Ok(())
}
