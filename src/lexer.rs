use std::io::BufRead;

pub enum Token {
    KeywordSet,          // set
    KeywordProc,         // proc
    KeywordIf,           // if
    KeywordElseIf,       // elseif
    KeywordElse,         // else
    KeywordSwitch,       // switch
    KeywordLog,          // log
    KeywordSnat,         // snat
    KeywordNode,         // node
    KeywordPool,         // pool
    KeywordSnatPool,     // snatpool
    KeywordReturn,       // return
    KeywordWhen,         // when
    DoubleColon,         // ::
    Colon,               // :
    LCurlyBracket,       // {
    RCurlyBracket,       // }
    LSquareBracket,      // [
    RSquareBracket,      // ]
    LParen,              // (
    RParen,              // )
    LAngleBracket,       // <
    RAngleBracket,       // >
    Dollar,              // $
    Quote,               // "
    Hash,                // #
    Minus,               // -
    Plus,                // +
    Equals,              // =
    Modulo,              // %
    Star,                // *
    Bang,                // !
    Ampersand,           // &
    Newline,             // \n
    Identifier(Vec<u8>), // [a-zA-Z0-9_\.]+
    Other(Vec<u8>),      // <lazy>
}

impl Token {
    fn is_keyword(&self) -> bool {
        match self {
            Token::KeywordSet
            | Token::KeywordProc
            | Token::KeywordIf
            | Token::KeywordElseIf
            | Token::KeywordElse
            | Token::KeywordSwitch
            | Token::KeywordLog
            | Token::KeywordSnat
            | Token::KeywordNode
            | Token::KeywordPool
            | Token::KeywordSnatPool
            | Token::KeywordReturn
            | Token::KeywordWhen => true,
            _ => false,
        }
    }

    fn is_operator(&self) -> bool {
        match self {
            Token::DoubleColon
            | Token::Minus
            | Token::Plus
            | Token::Equals
            | Token::Modulo
            | Token::Star
            | Token::Ampersand => true,
            _ => false,
        }
    }

    fn is_symbol(&self) -> bool {
        match self {
            Token::Colon | Token::Dollar | Token::Quote | Token::Bang => true,
            _ => false,
        }
    }

    fn is_bracket(&self) -> bool {
        match self {
            Token::LCurlyBracket
            | Token::RCurlyBracket
            | Token::LSquareBracket
            | Token::RSquareBracket
            | Token::LParen
            | Token::RParen
            | Token::LAngleBracket
            | Token::RAngleBracket => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum LexerFail {
    ExpectedString, // when parsing log
}

type Result<T> = std::result::Result<T, LexerFail>;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn lex(mut self, buf: Vec<u8>) -> Result<Vec<Token>> {
        for line in buf.lines() {
            self.lex_line(&Lexer::normalize(line.unwrap().as_bytes()))?; // lstrip & rstrip
            self.tokens.push(Token::Newline);
        }

        Ok(self.tokens)
    }

    fn lex_line(&mut self, mut line: &[u8]) -> Result<()> {
        while let Some((token, consumed)) = self.try_lex(line) {
            self.tokens.push(token);
            line = &line[consumed..];

            let n = self.tokens.len().saturating_sub(1);
            match (self.tokens.get(n.saturating_sub(1)), self.tokens.get(n)) {
                (_, Some(Token::Hash)) => {
                    self.tokens.push(Token::Other(Lexer::normalize(&line))); // lstrip only
                    return Ok(());
                }
                (Some(Token::KeywordLog), Some(Token::Identifier(_))) => {
                    dbg!(String::from_utf8_lossy(line));
                    let (value, consumed) =
                        Lexer::extract_string(line).ok_or(LexerFail::ExpectedString)?;
                    self.tokens.push(Token::Other(value.to_vec()));
                    line = &line[consumed..];
                }
                _ => {}
            }
        }
        if !line.is_empty() {
            println!("lexer: failed to read next");
            dbg!(self.tokens.last());
            dbg!(String::from_utf8_lossy(line));
            unreachable!();
        }
        Ok(())
    }

    fn try_lex(&self, line: &[u8]) -> Option<(Token, usize)> {
        let consumed = line
            .iter()
            .take_while(|&&x| is_whitespace_or_semicolon(x))
            .count();

        match &line[consumed..] {
            x if x.starts_with(b"snatpool") => Some((Token::KeywordSnatPool, 8)),
            x if x.starts_with(b"switch") => Some((Token::KeywordSwitch, 6)),
            x if x.starts_with(b"return") => Some((Token::KeywordReturn, 6)),
            x if x.starts_with(b"elseif") => Some((Token::KeywordElseIf, 6)),
            x if x.starts_with(b"else") => Some((Token::KeywordElse, 4)),
            x if x.starts_with(b"pool ") => Some((Token::KeywordPool, 4)),
            x if x.starts_with(b"node ") => Some((Token::KeywordNode, 4)),
            x if x.starts_with(b"proc ") => Some((Token::KeywordProc, 4)),
            x if x.starts_with(b"snat ") => Some((Token::KeywordSnat, 4)),
            x if x.starts_with(b"when ") => Some((Token::KeywordWhen, 4)),
            x if x.starts_with(b"log ") => Some((Token::KeywordLog, 3)),
            x if x.starts_with(b"set ") => Some((Token::KeywordSet, 3)),
            x if x.starts_with(b"if") => Some((Token::KeywordIf, 2)),
            x if x.starts_with(b"::") => Some((Token::DoubleColon, 2)),
            x if x.starts_with(b"[") => Some((Token::LSquareBracket, 1)),
            x if x.starts_with(b"]") => Some((Token::RSquareBracket, 1)),
            x if x.starts_with(b"{") => Some((Token::LCurlyBracket, 1)),
            x if x.starts_with(b"}") => Some((Token::RCurlyBracket, 1)),
            x if x.starts_with(b"(") => Some((Token::LParen, 1)),
            x if x.starts_with(b")") => Some((Token::RParen, 1)),
            x if x.starts_with(b"<") => Some((Token::LAngleBracket, 1)),
            x if x.starts_with(b">") => Some((Token::RAngleBracket, 1)),
            x if x.starts_with(b"#") => Some((Token::Hash, 1)),
            x if x.starts_with(b"$") => Some((Token::Dollar, 1)),
            x if x.starts_with(b"\"") => Some((Token::Quote, 1)),
            x if x.starts_with(b"-") => Some((Token::Minus, 1)),
            x if x.starts_with(b"+") => Some((Token::Plus, 1)),
            x if x.starts_with(b"=") => Some((Token::Equals, 1)),
            x if x.starts_with(b"%") => Some((Token::Modulo, 1)),
            x if x.starts_with(b"*") => Some((Token::Star, 1)),
            x if x.starts_with(b"!") => Some((Token::Bang, 1)),
            x if x.starts_with(b"&") => Some((Token::Ampersand, 1)),
            x if x.starts_with(b":") => Some((Token::Colon, 1)),
            b"" => None,
            x => {
                let identifier = Lexer::extract_identifier(x);
                let len = identifier.len();
                if len == 0 {
                    None
                } else {
                    Some((Token::Identifier(identifier), len))
                }
            }
        }
        .map(|(t, c)| (t, c + consumed))
    }

    // TODO: normalize = lstrip + rstrip
    fn normalize(line: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = line
            .into_iter()
            .rev()
            .skip_while(|&&x| is_whitespace_or_semicolon(x))
            .copied()
            .collect();
        buf.reverse();
        buf.into_iter()
            .skip_while(|&x| is_whitespace_or_semicolon(x))
            .collect()
    }

    fn extract_identifier(line: &[u8]) -> Vec<u8> {
        // assume line is lstripped
        line.into_iter()
            .take_while(|&&x| x.is_ascii_alphanumeric() || x == b'_' || x == b'.')
            .cloned()
            .collect()
    }

    fn extract_string(mut data: &[u8]) -> Option<(&[u8], usize)> {
        let consumed = data
            .iter()
            .take_while(|&&x| is_whitespace_or_semicolon(x))
            .count();
        data = &data[consumed..];

        let Some(b'"') = data.get(0) else { return None };
        let inside_len = data.iter().skip(1).take_while(|&&c| c != b'"').count();
        match data.get(inside_len + 1) {
            Some(b'"') => Some((&data[..=inside_len + 1], consumed + inside_len + 2)),
            _ => None,
        }
    }
}

fn is_whitespace_or_semicolon(symbol: u8) -> bool {
    match symbol {
        b' ' | b'\t' | b';' => true,
        _ => false,
    }
}

impl From<&Token> for Vec<u8> {
    fn from(val: &Token) -> Self {
        match val {
            Token::KeywordSet => b"set".to_vec(),
            Token::KeywordProc => b"proc".to_vec(),
            Token::KeywordIf => b"if".to_vec(),
            Token::KeywordElseIf => b"elseif".to_vec(),
            Token::KeywordElse => b"else".to_vec(),
            Token::KeywordSwitch => b"switch".to_vec(),
            Token::KeywordLog => b"log".to_vec(),
            Token::KeywordSnat => b"snat".to_vec(),
            Token::KeywordNode => b"node".to_vec(),
            Token::KeywordPool => b"pool".to_vec(),
            Token::KeywordSnatPool => b"snatpool".to_vec(),
            Token::KeywordReturn => b"return".to_vec(),
            Token::KeywordWhen => b"when".to_vec(),
            Token::DoubleColon => b"::".to_vec(),
            Token::LSquareBracket => b"[".to_vec(),
            Token::RSquareBracket => b"]".to_vec(),
            Token::LCurlyBracket => b"{".to_vec(),
            Token::RCurlyBracket => b"}".to_vec(),
            Token::LParen => b"(".to_vec(),
            Token::RParen => b")".to_vec(),
            Token::LAngleBracket => b"<".to_vec(),
            Token::RAngleBracket => b">".to_vec(),
            Token::Dollar => b"$".to_vec(),
            Token::Hash => b"#".to_vec(),
            Token::Newline => b"\n".to_vec(),
            Token::Identifier(data) => data.to_vec(),
            Token::Other(data) => data.to_vec(),
            Token::Quote => b"\"".to_vec(),
            Token::Minus => b"-".to_vec(),
            Token::Plus => b"+".to_vec(),
            Token::Equals => b"=".to_vec(),
            Token::Modulo => b"%".to_vec(),
            Token::Star => b"*".to_vec(),
            Token::Bang => b"!".to_vec(),
            Token::Ampersand => b"&".to_vec(),
            Token::Colon => b":".to_vec(),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(Vec::from(self)).expect("Failed to utf8 decode");
        match self {
            x if x.is_keyword() => write!(f, "kwrd:  \u{001b}[31m{}\u{001b}[0m", s),
            x if x.is_symbol() => write!(f, "sym:   \u{001b}[32m{}\u{001b}[0m", s),
            x if x.is_operator() => write!(f, "op:    \u{001b}[33m{}\u{001b}[0m", s),
            x if x.is_bracket() => write!(f, "brkt:  \u{001b}[34m{}\u{001b}[0m", s),
            Self::Newline => write!(f, "lf:    \u{001b}[1m\\n\u{001b}[0m"),
            Self::Other(_) => write!(f, "other: \u{001b}[36m{}\u{001b}[0m", s),
            Self::Identifier(_) => write!(f, "ident: {}", s),
            Self::Hash => write!(f, "hash:  \u{001b}[32m{}\u{001b}[0m", s),
            _ => {
                println!("{}", s);
                unreachable!()
            }
        }
    }
}
