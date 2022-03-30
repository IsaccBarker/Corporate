use crate::error;
use crate::instruction::Instruction;

use std::process;
use log::debug;

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
        let mut modify_ip = true;
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
                match stack.pop() {
                    Some(_) => (),
                    None => { debug!("Stack underflow"); error::print_error(); process::exit(-1); }
                };
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

