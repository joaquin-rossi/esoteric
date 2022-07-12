use crate::*;

pub fn compile(src: &str) -> Result<Vec<Instruction>, String> {
    let mut src = src.to_owned();
    src.retain(|c| "><+-,.[]".contains(c));

    let mut text = Vec::new();
    let brackets = pair_brackets(&src)?;

    for (i, c) in src.chars().enumerate() {
        let i = i as u32;
        let op = match c {
            '>' => IncPtr,
            '<' => DecPtr,
            '+' => IncData,
            '-' => DecData,
            ',' => Input,
            '.' => Output,
            '[' => ConJump(brackets[&i] + 1),
            ']' => IncJump(brackets[&i]),
            _ => panic!("unexpected instruction"),
        };

        text.push(op);
    }

    Ok(text)
}

fn pair_brackets(src: &str) -> Result<HashMap<u32, u32>, String> {
    let mut stack = Vec::<u32>::new();
    let mut map = HashMap::<u32, u32>::new();

    for (i, c) in src.chars().enumerate() {
        let i = i as u32;
        match c {
            '[' => stack.push(i),
            ']' => {
                let j = stack
                    .pop()
                    .ok_or("Unexpected closing bracket".to_string())?;
                map.insert(i, j);
                map.insert(j, i);
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        Ok(map)
    } else {
        Err("Some brackets where not closed".to_string())
    }
}
