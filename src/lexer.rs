use std::io::BufRead;

#[derive(Clone)]
pub enum Token {
    Keyword(Keyword), // set | proc | if | else | etc.
    LBracket,         // {
    RBracket,         // }
    Hash,             // #
    Newline,          // \n
    Other(Vec<u8>),   // lazy
}

#[derive(Clone, Copy)]
pub enum Keyword {
    Set,      //
    Proc,     //
    If,       //
    ElseIf,   //
    Else,     //
    Switch,   //
    Log,      //
    Snat,     //
    Node,     //
    Pool,     //
    SnatPool, //
    Return,   //
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn lex(mut self, buf: Vec<u8>) -> Vec<Token> {
        for line in buf.lines() {
            self.lex_line(&Lexer::normalize(line.unwrap().as_bytes())); // lstrip & rstrip
            self.tokens.push(Token::Newline);
        }

        self.tokens
    }

    fn lex_line(&mut self, mut line: &[u8]) {
        if line.is_empty() {
            self.tokens.push(Token::Newline);
            return;
        }

        match Lexer::try_keyword(&line) {
            Some((token, consumed)) => {
                self.tokens.push(token);
                line = &line[consumed..];
            }
            None => {}
        }

        while let Some((token, consumed)) = Lexer::try_char(line) {
            self.tokens.push(token);
            line = &line[consumed..];

            match self.tokens.last().unwrap() {
                Token::Hash => {
                    self.tokens.push(Token::Other(Lexer::normalize(&line))); // lstrip only
                    return;
                }
                _ => {}
            }
        }
        if line.len() != 0 {
            self.tokens.push(Token::Other(Lexer::normalize(line))); // lstrip only
        }
    }

    fn try_char(line_suffix: &[u8]) -> Option<(Token, usize)> {
        if line_suffix.is_empty() {
            return Some((Token::Newline, 0));
        }

        let consumed = line_suffix
            .iter()
            .take_while(|&&x| is_whitespace_or_semicolon(x))
            .count();

        match line_suffix.iter().skip(consumed).next() {
            Some(b'{') => Some(Token::LBracket),
            Some(b'}') => Some(Token::RBracket),
            Some(b'#') => Some(Token::Hash),
            _ => None,
        }
        .map(|t| (t, 1 + consumed))
    }

    fn try_keyword(line: &[u8]) -> Option<(Token, usize)> {
        let consumed = line
            .iter()
            .take_while(|&&x| is_whitespace_or_semicolon(x))
            .count();

        match line {
            x if x.starts_with(b"snatpool") => Some((Token::Keyword(Keyword::SnatPool), 8)),
            x if x.starts_with(b"switch") => Some((Token::Keyword(Keyword::Switch), 6)),
            x if x.starts_with(b"return") => Some((Token::Keyword(Keyword::Return), 6)),
            x if x.starts_with(b"elseif") => Some((Token::Keyword(Keyword::ElseIf), 6)),
            x if x.starts_with(b"else") => Some((Token::Keyword(Keyword::Else), 4)),
            x if x.starts_with(b"pool") => Some((Token::Keyword(Keyword::Pool), 4)),
            x if x.starts_with(b"node") => Some((Token::Keyword(Keyword::Node), 4)),
            x if x.starts_with(b"proc") => Some((Token::Keyword(Keyword::Proc), 4)),
            x if x.starts_with(b"snat") => Some((Token::Keyword(Keyword::Snat), 4)),
            x if x.starts_with(b"log") => Some((Token::Keyword(Keyword::Log), 3)),
            x if x.starts_with(b"set") => Some((Token::Keyword(Keyword::Set), 3)),
            x if x.starts_with(b"if") => Some((Token::Keyword(Keyword::If), 2)),
            _ => None,
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
}

fn is_whitespace_or_semicolon(symbol: u8) -> bool {
    match symbol {
        b' ' | b'\t' | b';' => true,
        _ => false,
    }
}
