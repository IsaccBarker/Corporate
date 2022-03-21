use crate::error;

use std::process;
use log::debug;

const PRINT_RE: &str = r#"\s*(apologize|state|tell|say|print)\s*/gmi"#;
const ADD_RE: &str = r#"\s*(add|include|\. with)\s*/gmi"#;
const SUB_RE: &str = r#"\s*(subtract|discontinue|ignore|\. without)\s*/gmi"#;
const MULT_RE: &str = r#"\s*(multiple|time|emphasize|boost|\. of)"\s*/gmi"#;
const DIV_RE: &str = r#"\s*(divide|controversial between|don't like|\. over)\s*/gmi"#;
const MOD_RE: &str = r#"\s*(remainder)\s*/gmi"#;
const POP_RE: &str = r#"\s*(I don't have the bandwidth to|let's drop)\s*/gmi"#;
const PUSH_RE: &str = r#"\s*(I'd like to consider|bring to attension)\s*/gmi"#;
const FUNCTION_CALL_RE: &str = r#"\s*(call|refer|proceed|follow through)\s*"#;
const IF_EQUALS_RE: &str = r#"\s*if .* (correct|affirmitive)\s*/gmi"#;
const IF_NEQUALS_RE: &str = r#"\s*if .* (not|false|incorrect)\s*/gmi"#;
const IF_LESS_RE: &str = r#"\s*if .* (doesn't|is worse than|fails to|let's .* down)\s*/gmi"#;
const IF_GREATER_RE: &str = r#"\s*if .* (exceeds|is better than|helps us)\s*/gmi"#;
const GOTO_RE: &str = r#"\s*(goto|go to|jump|arive at|recenter)\s*\gmi"#;

pub enum Instruction {
    Print,
    Add,
    Sub,
    Pop,
    Push { data: String },
    IfEquals,
    IfLess,
    Goto { to: usize },
}

fn get_int_from_stack(stack: &Vec<String>, get: usize) -> i32 {
    match stack.get(stack.len()-(1-get)) {
        Some(s) => match s.parse::<i32>() {
            Ok(s) => s,
            Err(e) => { debug!("Could not parse int"); error::print_error(); process::exit(-1); }
        },

        None => { debug!("Stack underflow"); error::print_error(); process::exit(-1); }
    }
}

fn get_from_stack(stack: &Vec<String>, offset: usize) -> String {
    match stack.get(stack.len()-(1-offset)) {
        Some(s) => s.to_string(),
        None => { debug!("Stack underflow"); error::print_error(); process::exit(-1); }
    }
}

fn get_a_b(stack: &Vec<String>, offset: usize) -> (i32, i32) {
    let a = get_int_from_stack(stack, 0);
    let b = get_int_from_stack(stack, 1);

    (a, b)
}

pub fn execute(instructions: &Vec<Instruction>) {
    let mut stack: Vec<String> = vec![];
    let mut ip: usize = 0;

    loop {
        let mut modify_ip = false;
        let instruction = match instructions.get(ip) {
            Some(instruction) => instruction,
            None => break,
        };

        match instruction {
            Instruction::Print => {
                println!("{}", get_from_stack(&stack, 0));
            }

            Instruction::Add => {
                let (a, b) = get_a_b(&stack, 0);

                stack.truncate(stack.len()-2);
                stack.push((a + b).to_string());
            }

            Instruction::Sub => {
                let (a, b) = get_a_b(&stack, 0);

                stack.truncate(stack.len()-2);
                stack.push((a - b).to_string());
            }

            Instruction::IfEquals => {
                let (a, b) = get_a_b(&stack, 1);

                if a == b {
                    ip = get_int_from_stack(&stack, 0) as usize;
                }
            }

            Instruction::IfLess => {
                let (a, b) = get_a_b(&stack, 1);

                if a < b {
                    ip = get_int_from_stack(&stack, 0) as usize;
                }
            }

            Instruction::Pop => {
                stack.pop();
            }

            Instruction::Push { data } => {
                stack.push(data.to_owned());
            }

            Instruction::Goto { to } => {
                ip = *to;

                modify_ip = false;
            }
        }

        if modify_ip {
            ip += 1;
        }
    }
}

