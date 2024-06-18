use std::io::BufRead;

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
    KeywordWhen,     // when
    LBracket,        // {
    RBracket,        // }
    Dollar,          // $
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

        if let Some((token, consumed)) = Lexer::try_keyword(&line) {
            self.tokens.push(token);
            self.lex_line(&line[consumed..]);
            return;
        }

        while !line.is_empty() {
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
            if line.is_empty() {
                return;
            }

            let mut buf = Vec::new();
            while Lexer::try_char(line).is_none() && !line.is_empty() {
                buf.push(line[0]);
                line = &line[1..];
            }
            self.tokens.push(Token::Other(Lexer::normalize(&buf))); // lstrip only
        }
    }

    fn try_char(line_suffix: &[u8]) -> Option<(Token, usize)> {
        let consumed = line_suffix
            .iter()
            .take_while(|&&x| is_whitespace_or_semicolon(x))
            .count();

        match line_suffix.iter().skip(consumed).next() {
            Some(b'{') => Some(Token::LBracket),
            Some(b'}') => Some(Token::RBracket),
            Some(b'#') => Some(Token::Hash),
            Some(b'$') => Some(Token::Dollar),
            _ => None,
            // TODO: '(' | ')' | '"' | '''
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
            x if x.starts_with(b"when") => Some((Token::KeywordWhen, 4)),
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
            Token::LBracket => b"{".to_vec(),
            Token::RBracket => b"}".to_vec(),
            Token::Dollar => b"$".to_vec(),
            Token::Hash => b"#".to_vec(),
            Token::Newline => b"\n".to_vec(),
            Token::Other(data) => data.to_vec(),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(Vec::from(self)).expect("Failed to utf8 decode");
        match self {
            Self::Newline => write!(f, "symbol: \u{001b}[1m\\n\u{001b}[0m"),
            Self::Other(_) => write!(f, "text: {}", s),
            Self::LBracket | Self::RBracket => write!(f, "symbol: \u{001b}[34m{}\u{001b}[0m", s),
            Self::Hash => write!(f, "symbol: \u{001b}[32m{}\u{001b}[0m", s),
            Self::Dollar => write!(f, "symbol: \u{001b}[33m{}\u{001b}[0m", s),
            _ => write!(f, "keyword: \u{001b}[31m{}\u{001b}[0m", s),
        }
    }
}
