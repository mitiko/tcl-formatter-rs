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
    When {
        event_name: Vec<u8>,
        body: Box<Ast>,
    },
    Statement(Statement),
    EmptyLine,
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
    Other { data: Vec<u8> },
}

impl std::fmt::Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(trees) => write!(f, "Ast::Block of {} trees", trees.len()),
            Self::Comment(_) => write!(f, "Ast::Comment"),
            Self::Procedure { parameters, .. } => {
                write!(f, "Ast::Procedure with {} parameters", parameters.len())
            }
            Self::If {
                condition_body_clauses,
                maybe_block_if_false,
            } => match (condition_body_clauses.len(), maybe_block_if_false) {
                (1, None) => write!(f, "Ast::If (if)"),
                (1, Some(_)) => write!(f, "Ast::If (if-else)"),
                (x, None) => write!(f, "Ast::If (if-elseif[{}])", x),
                (x, Some(_)) => write!(f, "Ast::If (if-elseif[{}]-else)", x),
            },
            Self::Switch { condition, .. } => {
                write!(f, "Ast::Switch with {} conditions", condition.len())
            }
            Self::Statement(s) => match s {
                Statement::Set { .. } => write!(f, "Ast::Statement::Set"),
                Statement::Log { .. } => write!(f, "Ast::Statement::Log"),
                Statement::Snat { .. } => write!(f, "Ast::Statement::Snat"),
                Statement::Node { .. } => write!(f, "Ast::Statement::Node"),
                Statement::Pool { .. } => write!(f, "Ast::Statement::Pool"),
                Statement::SnatPool { .. } => write!(f, "Ast::Statement::SnatPool"),
                Statement::Return { value } if value.is_some() => {
                    write!(f, "Ast::Statement::Return with value")
                }
                Statement::Return { .. } => write!(f, "Ast::Statement::Return"),
                Statement::Other { data } => {
                    write!(f, "Ast::Statement::Other with length {}", data.len())
                }
            },
            Self::EmptyLine => write!(f, "Ast::EmptyLine"),
            Self::When { event_name, .. } => write!(f, "Ast::When ({})", String::from_utf8_lossy(event_name)),
        }
    }
}
