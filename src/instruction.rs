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
    Mult,
    Div,
    Pop,
    Push { data: String },
    IfEquals,
    IfNequals,
    IfLess,
    IfGreater,
    Goto { to: usize },
}

pub fn execute(instructions: &Vec<Instruction>) {
    let mut stack: Vec<String> = vec![];
    let mut ip: usize = 0;

    loop {
        let instruction = instructions.get(ip).unwrap();
        let modify_ip = false;

        match instruction {
            Instruction::Print => {
                println!("{}", stack.get(stack.len()-1).unwrap());
            }

            Instruction::Add => {
                let a = stack.get(stack.len()-1).unwrap().parse::<i32>().unwrap();
                let b = stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap();

                stack.truncate(stack.len()-2);
                stack.push((a + b).to_string());
            }

            Instruction::Sub => {
                let a = stack.get(stack.len()-1).unwrap().parse::<i32>().unwrap();
                let b = stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap();

                stack.truncate(stack.len()-2);
                stack.push((a - b).to_string());
            }

            Instruction::Mult => {
                let a = stack.get(stack.len()-1).unwrap().parse::<i32>().unwrap();
                let b = stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap();

                stack.truncate(stack.len()-2);
                stack.push((a * b).to_string());
            }

            Instruction::Div => {
                let a = stack.get(stack.len()-1).unwrap().parse::<i32>().unwrap();
                let b = stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap();

                stack.truncate(stack.len()-2);
                stack.push((a / b).to_string());
            }

            Instruction::Pop => {
                stack.pop();
            }

            Instruction::Push { data } => {
                stack.push(data.to_owned());
            }

            Instruction::IfEquals => {
                if stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap() == stack.get(stack.len()-3).unwrap().parse::<i32>().unwrap() {
                    ip = stack.get(stack.len()-1).unwrap().parse::<usize>().unwrap();
                }
            }

            Instruction::IfNequals => {
                if stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap() != stack.get(stack.len()-3).unwrap().parse::<i32>().unwrap() {
                    ip = stack.get(stack.len()-1).unwrap().parse::<usize>().unwrap();
                }
            }

            Instruction::IfLess => {
                if stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap() < stack.get(stack.len()-3).unwrap().parse::<i32>().unwrap() {
                    ip = stack.get(stack.len()-1).unwrap().parse::<usize>().unwrap();
                }
            }

            Instruction::IfGreater => {
                if stack.get(stack.len()-2).unwrap().parse::<i32>().unwrap() > stack.get(stack.len()-3).unwrap().parse::<i32>().unwrap() {
                    ip = stack.get(stack.len()-1).unwrap().parse::<usize>().unwrap();
                }
            }

            Instruction::Goto { to } => {
                ip = *to;
            }
        }

        if modify_ip {
            ip += 1;
        }
    }
}

