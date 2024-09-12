use miette::{Error};
use std::usize;

pub struct Lexer<'de> {
    whole: &'de str,
    rest : &'de str,
    byte : usize,
    peeked: Option<Result<Token<'de>, miette::Error>>,
}
impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte : 0,
            peeked : None
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.byte+=c.len_utf8();
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
