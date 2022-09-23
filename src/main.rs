use crate::parser::read_file;

mod parser;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Push(u32),
    Add,
    Sub,
    Mul,
    Div,
    PrintInt,
    PrintStr,
    ForLoop
}

#[derive(Debug, Clone)]
struct Vm {
    stack: Vec<u32>,
    program: Vec<Op>
}

impl Vm {
    fn push(&mut self, value: u32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> u32 {
        match self.stack.pop() {
            Some(n) => n,
            None => panic!("Not enough items on the stack!")
        }
    }

    fn execute_op(&mut self, op: Op, i: usize) {
        use Op::*;

        match op {
            Push(n) => self.push(n),
            Add => {
                let b = self.pop();
                let a = self.pop();
                self.push(a+b);
            }
            Sub => {
                let b = self.pop();
                let a = self.pop();
                self.push(a-b);
            }
            Mul => {
                let b = self.pop();
                let a = self.pop();
                self.push(a*b);
            }
            Div => {
                let b = self.pop();
                let a = self.pop();
                self.push(a/b);
            }
            PrintInt => {
                let a = self.pop();
                println!("{}", a);
            }
            PrintStr => {
                let a = self.pop();
                for _ in 0..a {
                    print!("{}", self.pop() as u8 as char);
                }
                println!();
            }
            ForLoop => {
                let b = self.pop();
                let a = self.pop() as usize;
                for _ in 1..b {
                    if (i as isize) < (3-a as isize) { panic!("Not enough items on the stack!") }
                    for (i, op) in self.clone().program[i-3-a..=i-3].iter().enumerate() {
                        self.execute_op(*op, i);
                    }
                }
            }
        }
    }

    fn execute_program(&mut self) {
        if self.program.contains(&Op::ForLoop) { println!("Warning! You are using a for loop in your program!\nThis is a expirimental feature and hasn't been tested yet!") }
        for (i, op) in self.clone().program.into_iter().enumerate() {
            self.execute_op(op, i);
        }
    }
}

fn main() {
    let program = match read_file("amongus.txt") {
        Ok(program) => program,
        Err(_) => panic!("Couldn't read input file!")
    };
    let mut vm = Vm { stack: vec![], program };
    vm.execute_program();
}