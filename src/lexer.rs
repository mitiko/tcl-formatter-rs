use std::io::BufRead;

#[derive(Clone)]
pub enum Token {
    KeywordSet,      // set
    KeywordProc,     // proc
    KeywordIf,       // if
    KeywordElseIf,   // elseif
    KeywordElse,     // else
    KeywordSwitch,   // switch
    KeywordLog,      // log
    KeywordSnat,     // snat
    KeywordNode,     // node
    KeywordPool,     // pool
    KeywordSnatPool, // snatpool
    KeywordReturn,   // return
    LBracket,        // {
    RBracket,        // }
    Hash,            // #
    Newline,         // \n
    Other(Vec<u8>),  // <lazy>
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
            x if x.starts_with(b"snatpool") => Some((Token::KeywordSnatPool, 8)),
            x if x.starts_with(b"switch") => Some((Token::KeywordSwitch, 6)),
            x if x.starts_with(b"return") => Some((Token::KeywordReturn, 6)),
            x if x.starts_with(b"elseif") => Some((Token::KeywordElseIf, 6)),
            x if x.starts_with(b"else") => Some((Token::KeywordElse, 4)),
            x if x.starts_with(b"pool") => Some((Token::KeywordPool, 4)),
            x if x.starts_with(b"node") => Some((Token::KeywordNode, 4)),
            x if x.starts_with(b"proc") => Some((Token::KeywordProc, 4)),
            x if x.starts_with(b"snat") => Some((Token::KeywordSnat, 4)),
            x if x.starts_with(b"log") => Some((Token::KeywordLog, 3)),
            x if x.starts_with(b"set") => Some((Token::KeywordSet, 3)),
            x if x.starts_with(b"if") => Some((Token::KeywordIf, 2)),
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