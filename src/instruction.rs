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

