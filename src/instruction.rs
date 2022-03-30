use crate::email::Email;
use crate::error;

use regex::Regex;
use log::debug;
use std::process;
use std::collections::BTreeMap;

const PRINT_RE: &str = r#"(?i)\s*(apologize|state|tell|say|print)\s*"#;
const ADD_RE: &str = r#"(?i)\s*(add|include|\. with)\s*"#;
const SUB_RE: &str = r#"(?i)\s*(subtract|discontinue|ignore|\. without)\s*"#;
const POP_RE: &str = r#"(?i)\s*(I don't have the bandwidth to|let's drop)\s*"#;
const PUSH_RE: &str = r#"(?i)\s*(I'd like to consider|bring to attension)\s*"#;
const FUNCTION_CALL_RE: &str = r#"(?i)\s*(call|refer|proceed|follow through)\s*"#;
const IF_EQUALS_RE: &str = r#"(?i)\s*if .* (correct|affirmitive)\s*"#;
const IF_LESS_RE: &str = r#"(?i)\s*if .* (doesn't|is worse than|fails to|let's .* down)\s*"#;

#[derive(Debug)]
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

fn get_index_from_match(cap: &regex::Captures) -> usize {
    cap.get(0).unwrap().start()
}

pub fn get_instructions(emails: &Vec<Email>) -> Vec<Instruction> {
    let mut intermediate: BTreeMap<usize, Instruction> = BTreeMap::new();
    let mut instructions: Vec<Instruction> = vec![];
    let unresolved_instructions: Vec<Instruction> = vec![];
    let print_re = Regex::new(PRINT_RE).unwrap();
    let add_re = Regex::new(ADD_RE).unwrap();
    let sub_re = Regex::new(SUB_RE).unwrap();
    let push_re = Regex::new(PUSH_RE).unwrap();
    let pop_re = Regex::new(POP_RE).unwrap();
    let function_call_re = Regex::new(FUNCTION_CALL_RE).unwrap();
    let if_equals_re = Regex::new(IF_EQUALS_RE).unwrap();
    let if_less_re = Regex::new(IF_LESS_RE).unwrap();

    for email in emails {
        for paragraph in &email.paragraphs {
            for cap in print_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Print);
            }

            for cap in add_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Add);
            }

            for cap in sub_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Sub);
            }

            for cap in push_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Push { data: "TODO".to_string() });
            }

            for cap in pop_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Pop);
            }

            for cap in function_call_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::Goto { to: 420 });
            }

            for cap in if_equals_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::IfEquals);
            }

            for cap in if_less_re.captures_iter(&paragraph) {
                intermediate.insert(get_index_from_match(&cap), Instruction::IfLess);
            }

            println!("{:?}", intermediate);
            intermediate.values().iter().for_each(|k| instructions.push(k));
            intermediate.clear();
            // instructions.append(intermediate.keys().collect::<Vec<&mut Instruction>>());
        }
    }

    if unresolved_instructions.len() != 0 {
        debug!("Found {} unresolved instructions:\n{:?}",
               unresolved_instructions.len(),
               unresolved_instructions);
        error::print_error();

        process::exit(-1);
    }

    debug!("Instructions:\n{:?}", instructions);

    instructions
}

