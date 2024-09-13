use miette::Error;
use std::{fmt::Display, usize};

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize,
}
impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.byte += c.len_utf8();
        self.rest = chars.as_str();
        match c {
            '$' => return Some(Ok(Token::DollarSign)),
            '+' => return Some(Ok(Token::Addition)),
            '-' => return Some(Ok(Token::Substraction)),
            '*' => return Some(Ok(Token::Multiplication)),
            '/' => return Some(Ok(Token::Division)),
            '\\' => return Some(Ok(Token::AntiSlash)),
            '?' => return Some(Ok(Token::QuestionMark)),
            '!' => return Some(Ok(Token::Bang)),
            '"' => return Some(Ok(Token::QuotationMark)),
            '\'' => return Some(Ok(Token::Apostrophe)),
            ':' => return Some(Ok(Token::Colon)),
            '.' => return Some(Ok(Token::Point)),
            '<' => return Some(Ok(Token::LessThan)),
            '=' => return Some(Ok(Token::Equal)),
            '>' => return Some(Ok(Token::GreaterThan)),
            '[' => return Some(Ok(Token::LeftSquareBracket)),
            ']' => return Some(Ok(Token::RightSquareBracket)),
            '(' => return Some(Ok(Token::LeftParnathesis)),
            ')' => return Some(Ok(Token::RightParnathesis)),
            '^' => return Some(Ok(Token::Caret)),
            '#' => return Some(Ok(Token::Pound)),
            '@' => return Some(Ok(Token::AtSign)),
            '%' => return Some(Ok(Token::Ampersand)),
            ',' => return Some(Ok(Token::Comma)),
            ';' => return Some(Ok(Token::SemiColon)),
            '{' => return Some(Ok(Token::LeftBracket)),
            '}' => return Some(Ok(Token::RightBracket)),
            '~' => return Some(Ok(Token::Tilde)),
            //we ignore comments

            //'a'..='z' | 'A'..='Z' => {
            //    return Some(Ok(Token::Charcter(a_token.unwrap()));

            //}
            //'0'..='9' => {
            //    return Some(Ok(Token::Number(a_token.unwrap()));

            //}
            _ => unreachable!("this token is not valid"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'de> {
    DollarSign,         //         $
    Addition,           //         +
    Substraction,       //         -
    Multiplication,     //         *
    Division,           //         /
    AntiSlash,          //         \
    QuestionMark,       //         ?
    Bang,               //
    Apostrophe,         //         '
    QuotationMark,      //         "
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
    Tilde,
    RightBracket, //         }
    Literal(&'de str),
}
impl<'de> Display for Token<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::DollarSign => write!(f, " $ "),
            Token::Addition => write!(f, " + "),
            Token::Substraction => write!(f, " - "),
            Token::Multiplication => write!(f, " * "),
            Token::Division => write!(f, " / "),
            Token::AntiSlash => write!(f, r" \ "),
            Token::QuestionMark => write!(f, " ? "),
            Token::Bang => write!(f, " ! "),
            Token::Apostrophe => write!(f, " ' "),
            Token::QuotationMark => write!(f, " \" "),
            Token::Colon => write!(f, " : "),
            Token::Point => write!(f, " . "),
            Token::LessThan => write!(f, " < "),
            Token::Equal => write!(f, " = "),
            Token::GreaterThan => write!(f, " > "),
            Token::LeftSquareBracket => write!(f, " [ "),
            Token::RightSquareBracket => write!(f, " ] "),
            Token::LeftParnathesis => write!(f, " ( "),
            Token::RightParnathesis => write!(f, " ) "),
            Token::Caret => write!(f, " ^ "),
            Token::Pound => write!(f, " # "),
            Token::AtSign => write!(f, " @ "),
            Token::Ampersand => write!(f, " % "),
            Token::Comma => write!(f, " , "),
            Token::SemiColon => write!(f, " ; "),
            Token::LeftBracket => write!(f, " {{ "),
            Token::Tilde => write!(f, " ~ "),
            Token::RightBracket => write!(f, " }} "),
            Token::Literal(_) => write!(f, "not implemeneted yet!"),
        }
    }
}
