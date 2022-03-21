const PRINT_RE: &str = r#"\s+(apologize|state|tell|say|print)\s+/gmi"#;
const ADD_RE: &str = r#"\s+(add|include|\. with)\s+/gmi"#;
const SUB_RE: &str = r#"\s+(subtract|discontinue|ignore|\. without)\s+/gmi"#;
const MULT_RE: &str = r#"\s+(multiple|time|emphasize|boost|\. of)"\s+/gmi"#;
const DIV_RE: &str = r#"\s+(divide|controversial between|don't like|\. over)\s+/gmi"#;
const MOD_RE: &str = r#"\s+(remainder)\s+/gmi"#;
const POP_RE: &str = r#"\s+(I don't have the bandwidth to|let's drop)\s+/gmi"#;
const PUSH_RE: &str = r#"\s+(I'd like to consider|bring to attension)\s+/gmi"#;

// (.+see the <function name>.+ interation stated (yesterday|today|.+ago).+)
//

pub enum Instruction {
    CallFunction { class: String, function: String },
    PushStack { things: Vec<String> },
    PopStack { num: i32 },
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Print,
}

