// Assume all Vec<u8> in ASTs are l-stripped and r-stripped of '\s\t' | ';'
pub enum Ast {
    Block(Vec<Ast>),  // list of ASTs
    Comment(Vec<u8>), // text after #
    Procedure {
        name: Vec<u8>,
        parameters: Vec<Vec<u8>>,
        body: Box<Ast>,
    },
    If {
        condition_body_clauses: Vec<(Vec<u8>, Ast)>,
        maybe_block_if_false: Option<Box<Ast>>,
    },
    Switch {
        condition: Vec<u8>,
        value_block_or_fallthrough_vec: Vec<(Vec<u8>, Option<Ast>)>,
    },
    Statement(Statement),
    Newline,
    // TODO: GTP/UDP func calls
}

pub enum Statement {
    Set { identifier: Vec<u8>, value: Vec<u8> },
    Log { bucket: Vec<u8>, value: Vec<u8> },
    Snat { ip_address: Vec<u8>, port: Vec<u8> },
    Node { ip_address: Vec<u8>, port: Vec<u8> },
    Pool { identifier: Vec<u8> },
    SnatPool { identifier: Vec<u8> },
    Return { value: Option<Vec<u8>> },
    // Other { data: Vec<u8> },
}
