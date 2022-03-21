const PRINT_RE: &str = r#"\s*(apologize|state|tell|say|print)\s*/gmi"#;
const ADD_RE: &str = r#"\s*(add|include|\. with)\s*/gmi"#;
const SUB_RE: &str = r#"\s*(subtract|discontinue|ignore|\. without)\s*/gmi"#;
const MULT_RE: &str = r#"\s*(multiple|time|emphasize|boost|\. of)"\s*/gmi"#;
const DIV_RE: &str = r#"\s*(divide|controversial between|don't like|\. over)\s*/gmi"#;
const MOD_RE: &str = r#"\s*(remainder)\s*/gmi"#;
const POP_RE: &str = r#"\s*(I don't have the bandwidth to|let's drop)\s*/gmi"#;
const PUSH_RE: &str = r#"\s*(I'd like to consider|bring to attension)\s*/gmi"#;
// This one is pretty complex, so I've provided a link here
//     https://regex101.com/r/nEiQLI/1
// const FUNCTION_CALL_RE: &str = r#"\s*((.*see the (last )?(meeting|conference|chat|call|note|letter|download|uplink|conversation|consultation).+referenced (yesterday|today|in this email|.+ ago).+)|(, namely .* (last )?(meeting|conference|chat|call|note|letter|download|uplink|conversation|consultation).+referenced (yesterday|today|in this email|.+ ago).*))\s*/gmi"#;
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
    FunctionCall,
    IfEquals,
    IfNequals,
    IfLess,
    IfGreater,
    Goto,
}

