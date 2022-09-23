use std::{fs::File, io::{Read, BufReader}};
use crate::Op;

pub fn read_file(filename: &str) -> std::io::Result<Vec<Op>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut tmp_program: Vec<u32> = vec![];
    let mut program: Vec<Op> = vec![];
    reader.read_to_string(&mut buffer)?;
    'lines: for line in buffer.lines() {
        let mut amonguses: u32 = 0;
        for amongus in line.split_whitespace() {
            if amongus == "amongus" {
                amonguses += 1;
            }  else if amongus == "//" {
                continue 'lines;
            } else {
                panic!("Non-Amongus instruction!");
            }
        }
        tmp_program.push(amonguses);
    }

    let mut i = 0;
    loop {
        if i >= tmp_program.len() { break }
        let end = i+1 == tmp_program.len();

        let op = match tmp_program[i] {
            1 => {
                if end { panic!("Push Op on EOF") }
                Op::Push(tmp_program[i+1])

            },
            2 => Op::Add,
            3 => Op::Sub,
            4 => Op::Mul,
            5 => Op::Div,
            6 => Op::PrintInt,
            7 => Op::PrintStr,
            8 => Op::ForLoop,
            _ => panic!("ParseError u8 to Op!")
        };

        program.push(op);
        if let Op::Push(_) = op { i += 2 } else { i += 1 };
    }

    Ok(program)
}