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
        if self.byte>=self.whole.len(){
            println!("reached the end of the file");
            return Some(Ok(Token::EOF))
        }
        loop {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.byte += c.len_utf8();
        self.rest = &self.whole[self.byte..];

        enum Started{
            Input,
            Character,
            String,
            Number,
            IDent,
        }
         let started = match c {
            ' '=> continue,
            '$' => return Some(Ok(Token::DollarSign)),
            '+' => return Some(Ok(Token::Addition)),
            '-' => return Some(Ok(Token::Substraction)),
            '*' => return Some(Ok(Token::Multiplication)),
            '/' => return Some(Ok(Token::Division)),
            '\\' => return Some(Ok(Token::AntiSlash)),
            '!' => return Some(Ok(Token::Bang)),
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
            '~' => {
                let endline= self.rest.find('\n').unwrap();
                let comment = &self.rest[c.len_utf8()..endline];
                self.byte+=endline+c.len_utf8()-1;
                self.rest=&self.whole[c.len_utf8()+self.byte-1..];
                //self.byte+=
                return Some(Ok(Token::Tilde(&comment)))
            },
            '?' => Started::Input,                  //return Some(Ok(Token::QuestionMark)),
            '"' =>  Started::String,                //return Some(Ok(Token::QuotationMark)),
            '\'' => Started::Character,                // return Some(Ok(Token::Apostrophe)),
            '0'..='9' => Started::Number,
            'a'..='z'|'A'..='Z' => Started::IDent,
            _ => { //println!("End of File {}",c);
                    continue
                },
        };

            match started{
                Started::String =>{ if let Some(end)= self.rest.find('"'){
                let literal=&self.rest[c.len_utf8()-1..end];
                self.byte+=literal.len()+c.len_utf8();
                self.rest=&self.whole[self.byte..];

                return Some(Ok(Token::Literal(literal)));
            }else {
                       //miette::errror here not implemented yet!
                    }
                },
                Started::IDent =>{
                    let mut whole:Vec<char>=Vec::new();
                    whole.push(c);
                    let rst = chars.take_while(|v| v.is_digit(10) || v.is_alphabetic()).collect::<Vec<char>>();
                    for i in 0..rst.len(){
                        whole.push(rst[i]);
                    }
                    self.byte+=whole.len()-1;
                    self.rest=&self.whole[self.byte..];

                    return Some(Ok(Token::VarIdentifier(&self.whole[self.byte-whole.len() ..self.byte])));
                },
               Started::Character => return Some(Ok(Token::Char(c))),
               Started::Input => {if c.is_digit(10){
                              return Some(Ok(Token::InputNumber(c.to_digit(10).unwrap() as usize)));
                           }else {return Some(Ok(Token::InputChar(c)))}
            },
              Started::Number => { let mut whole:Vec<char>=Vec::new();
                              whole.push(c);
                       let rst = chars.take_while(|v| v.is_digit(10)).collect::<Vec<char>>();
                       for i in 0..rst.len(){
                      whole.push(rst[i]);
                }
                     self.byte+=whole.len()-1;
                     self.rest=&self.whole[self.byte..];
                   let whole = whole.iter().collect::<String>();

                return Some(Ok(Token::Number(whole.parse::<usize>().unwrap())));
            },
            };
        return None;
        }
    }
    }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'de> {
    EOF,
    //Whitespace,
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
    VarIdentifier(&'de str),
    Literal(&'de str),
    Char(char),
    Number(usize),
    InputChar(char),
    InputNumber(usize),

}
impl<'de> Display for Token<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF=> write!(f,"EOF->end of file"),
            Token::DollarSign => write!(f, "DollarSign->$"),
            //Token::Whitespace => write!(f, "Whitespace->Whitespace"),
            Token::Addition => write!(f, "Addition->+"),
            Token::Substraction => write!(f, "Substraction->-"),
            Token::Multiplication => write!(f, "Multiplication->*"),
            Token::Division => write!(f, "Division->/"),
            Token::AntiSlash => write!(f, r"AntiSlash->\"),
            Token::Bang => write!(f, "Bang->!"),
            Token::Colon => write!(f, "Colon->:"),
            Token::Point => write!(f, "Point->."),
            Token::LessThan => write!(f, "LessThan-><"),
            Token::Equal => write!(f, "Equal->="),
            Token::GreaterThan => write!(f, "GreaterThan->>"),
            Token::LeftSquareBracket => write!(f, "LeftSquareBracket->["),
            Token::RightSquareBracket => write!(f, "RightSquareBracket->]"),
            Token::LeftParnathesis => write!(f, "LeftParnathesis->("),
            Token::RightParnathesis => write!(f, "RightParnathesis->)"),
            Token::Caret => write!(f, "Caret->^"),
            Token::Pound => write!(f, "Pound->#"),
            Token::AtSign => write!(f, "AtSign->@"),
            Token::Ampersand => write!(f, "Ampersand->%"),
            Token::Comma => write!(f, "Comma->,"),
            Token::SemiColon => write!(f, "SemiColon->;"),
            Token::LeftBracket => write!(f, "LeftBracket->{{"),
            Token::Tilde(s) => write!(f, "Tilde->~comment_is->{}",s),
            Token::RightBracket => write!(f, "RightBracket->}}"),
            Token::Literal(s) => write!(f,"Literal->\"{}\"",s),
            Token::VarIdentifier(i) => write!(f,"VarIdentifier->{}",i),
            Token::Char(c) => write!(f,"Char->'{}",c),
            Token::Number(u) => write!(f,"Number->{}",u),
            Token::InputChar(c) => write!(f,"InputChar->?'{}",c),
            Token::InputNumber(u) => write!(f,"InputNumber->?{}",u),


        }
    }
}
