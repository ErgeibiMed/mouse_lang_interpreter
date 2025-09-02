use std::fmt::Display;

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize,
    line_number: usize,
}
impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
            line_number: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Result<TokenInfo, LexerError>> {
        self.into_iter().collect()
    }
}

pub struct TokenInfo<'de> {
    pub token: Token<'de>,
    pub pos: usize,
    pub line: usize,
}

#[derive(Debug)]
pub struct LexerError {
    pub lex_err: String,
    pub line: usize,
    pub pos: usize,
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<TokenInfo<'de>, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        //}
        //loop {
        let mut chars = self.rest.chars();
        while let Some(c) = chars.next() {
            if self.byte >= self.whole.len() {
                println!("reached the end of the file");
                return Some(Ok(TokenInfo {
                    token: Token::EOF,
                    pos: self.byte - 1,
                    line: self.line_number,
                }));
            }
            self.byte += c.len_utf8();
            self.rest = &self.whole[self.byte..];

            enum Started {
                String,
                Number,
            }
            let start = match c {
                ' ' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Whitespace,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }));
                }
                '\n' => {
                    self.line_number += 1;
                    continue;
                }
                '$' => {
                    return Some(Ok(TokenInfo {
                        token: Token::DollarSign,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '+' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Addition,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '-' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Substraction,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '*' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Multiplication,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '/' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Division,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '\\' => {
                    return Some(Ok(TokenInfo {
                        token: Token::AntiSlash,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '!' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Bang,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                ':' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Colon,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '.' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Point,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '<' => {
                    return Some(Ok(TokenInfo {
                        token: Token::LessThan,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '=' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Equal,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '>' => {
                    return Some(Ok(TokenInfo {
                        token: Token::GreaterThan,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '[' => {
                    return Some(Ok(TokenInfo {
                        token: Token::LeftSquareBracket,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                ']' => {
                    return Some(Ok(TokenInfo {
                        token: Token::RightSquareBracket,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '(' => {
                    return Some(Ok(TokenInfo {
                        token: Token::LeftParnathesis,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                ')' => {
                    return Some(Ok(TokenInfo {
                        token: Token::RightParnathesis,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '^' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Caret,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '#' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Pound,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '@' => {
                    return Some(Ok(TokenInfo {
                        token: Token::AtSign,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '%' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Ampersand,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                ',' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Comma,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                ';' => {
                    return Some(Ok(TokenInfo {
                        token: Token::SemiColon,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '{' => {
                    return Some(Ok(TokenInfo {
                        token: Token::LeftBracket,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '}' => {
                    return Some(Ok(TokenInfo {
                        token: Token::RightBracket,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '~' => {
                    let endline = self.rest.find('\n').unwrap();
                    //Comment always strat with tilde
                    //and end at the newline
                    //the comment doesnt include tilde
                    let starting_pos = self.byte - 1;
                    let comment = &self.rest[c.len_utf8()..endline];
                    self.byte += comment.len() + c.len_utf8();
                    self.rest = &self.whole[self.byte..];
                    self.line_number += 1;
                    return Some(Ok(TokenInfo {
                        token: Token::Tilde(&comment),
                        pos: starting_pos,
                        line: self.line_number,
                    }));
                }
                '?' => {
                    return Some(Ok(TokenInfo {
                        token: Token::QuestionMark,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '"' => Started::String, //return Some(Ok(Token::QuotationMark)),
                '\'' => {
                    return Some(Ok(TokenInfo {
                        token: Token::Apostrophe,
                        pos: self.byte - 1,
                        line: self.line_number,
                    }))
                }
                '0'..='9' => Started::Number,
                'a'..='z' | 'A'..='Z' => {
                    return {
                        Some(Ok(TokenInfo {
                            token: Token::Char(c),
                            pos: self.byte - 1,
                            line: self.line_number,
                        }))
                    }
                }
                unkown_token => {
                    let uknt = unkown_token.is_ascii_punctuation();
                    let err = format!("UnknownToken ({uknt}) at pos {pos}", pos = self.byte,);
                    return Some(Err(LexerError {
                        lex_err: err,
                        line: self.line_number,
                        pos: self.byte - 1,
                    }));
                }
            };

            match start {
                Started::String => {
                    if let Some(end) = self.rest.find('"') {
                        let literal = &self.rest[self.byte - 1..end];
                        let starting_pos = self.byte - 1;
                        self.byte += literal.len() + 1;
                        self.rest = &self.whole[self.byte..];

                        return Some(Ok(TokenInfo {
                            token: Token::Literal(literal),
                            pos: starting_pos,
                            line: self.line_number,
                        }));
                    } else {
                        let err = format!(
                            "string is not properly quoted at position {pos}",
                            pos = self.byte,
                        );
                        return Some(Err(LexerError {
                            lex_err: err,
                            line: self.line_number,
                            pos: self.byte - 1,
                        }));
                    }
                }
                Started::Number => {
                    let mut whole: Vec<char> = Vec::new();
                    whole.push(c);
                    let rst = chars.take_while(|v| v.is_digit(10)).collect::<Vec<char>>();
                    for i in 0..rst.len() {
                        whole.push(rst[i]);
                    }
                    let starting_pos = self.byte - 1;
                    self.byte += whole.len() - 1;
                    self.rest = &self.whole[self.byte..];
                    let whole = whole.iter().collect::<String>();

                    return Some(Ok(TokenInfo {
                        token: Token::Number(whole.parse::<usize>().unwrap()),
                        pos: starting_pos,
                        line: self.line_number,
                    }));
                }
            };
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'de> {
    EOF,
    Whitespace,
    DollarSign,         //         $
    Addition,           //         +
    Substraction,       //         -
    Multiplication,     //         *
    Division,           //         /
    AntiSlash,          //         \
    Bang,               //
    Colon,              //         :
    Point,              //         .
    LessThan,           //         <
    Equal,              //         =
    GreaterThan,        //         >
    LeftSquareBracket,  //         [
    RightSquareBracket, //         ]
    LeftParnathesis,    //         (
    RightParnathesis,   //         )
    Caret,              //         ^
    Pound,              //         #
    AtSign,             //         @
    Ampersand,          //         %
    Comma,              //         ,
    SemiColon,          //         ;
    LeftBracket,        //         {
    Tilde(&'de str),
    RightBracket, //         }
    QuestionMark,
    Literal(&'de str),
    Char(char),
    Number(usize),
    Apostrophe,
}
impl<'de> Display for Token<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF end of file"),
            Token::DollarSign => write!(f, "DollarSign $"),
            Token::QuestionMark => write!(f, "QuestionMark ?"),
            Token::Whitespace => write!(f, "Whitespace Whitespace"),
            Token::Addition => write!(f, "Addition +"),
            Token::Substraction => write!(f, "Substraction -"),
            Token::Multiplication => write!(f, "Multiplication *"),
            Token::Division => write!(f, "Division /"),
            Token::AntiSlash => write!(f, r"AntiSlash \"),
            Token::Bang => write!(f, "Bang !"),
            Token::Colon => write!(f, "Colon :"),
            Token::Point => write!(f, "Point ."),
            Token::LessThan => write!(f, "LessThan <"),
            Token::Equal => write!(f, "Equal ="),
            Token::GreaterThan => write!(f, "GreaterThan >"),
            Token::LeftSquareBracket => write!(f, "LeftSquareBracket ["),
            Token::RightSquareBracket => write!(f, "RightSquareBracket ]"),
            Token::LeftParnathesis => write!(f, "LeftParnathesis ("),
            Token::RightParnathesis => write!(f, "RightParnathesis )"),
            Token::Caret => write!(f, "Caret ^"),
            Token::Pound => write!(f, "Pound #"),
            Token::AtSign => write!(f, "AtSign @"),
            Token::Ampersand => write!(f, "Ampersand %"),
            Token::Comma => write!(f, "Comma ,"),
            Token::SemiColon => write!(f, "SemiColon ;"),
            Token::LeftBracket => write!(f, "LeftBracket {{"),
            Token::Tilde(s) => write!(f, "Tilde ~comment_is {}", s),
            Token::RightBracket => write!(f, "RightBracket }}"),
            Token::Literal(s) => write!(f, "Literal \"{}\"", s),
            Token::Char(c) => write!(f, "Char {}", c),
            Token::Number(u) => write!(f, "Number {}", u),
            Token::Apostrophe => write!(f, "Apostrophe '",),
        }
    }
}
