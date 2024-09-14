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
        loop{
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.byte += c.len_utf8();
        self.rest = chars.as_str();

        enum Started{
            Input,
            Character,
            String,
            Number,
            IDent,
        }
         let started = match c {
            ' ' => {continue;  },//no action
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
                     let comment = &self.rest[c.len_utf8()+1..endline+c.len_utf8()-1];
                     self.byte+=comment.len();
                    self.rest=&self.rest[self.byte..];
                    //self.byte+=
                    return Some(Ok(Token::Tilde))
                },
            '?' => Started::Input,                  //return Some(Ok(Token::QuestionMark)),
            '"' =>  Started::String,                //return Some(Ok(Token::QuotationMark)),
            '\'' => Started::Character,                // return Some(Ok(Token::Apostrophe)),
            '0'..='9' => Started::Number,
            'a'..='z'|'A'..='Z' => Started::IDent,
            _ => unreachable!("this token is not valid"),
        };

            match started{
                Started::String =>{ if let Some(end)= self.rest.find('"'){
                let literal=&self.rest[c.len_utf8()-1..end+c.len_utf8()-1];
                self.byte+=literal.len();
                self.rest=&self.rest[self.byte..];

                return Some(Ok(Token::Literal(literal)));
            }else {

                    }
                },
                Started::IDent =>{
                       let end= self.rest.find(' ').unwrap();
                      let identifier= &self.rest[c.len_utf8()..end+c.len_utf8()-1];
                      self.byte+=identifier.len();
                      self.rest=&self.rest[self.byte..];
                    return Some(Ok(Token::VarIdentifier(identifier)));
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
                     self.byte+=whole.len();
                     self.rest=&self.rest[self.byte..];
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
    Tilde,
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
            Token::DollarSign => write!(f, " $ "),
            Token::Addition => write!(f, " + "),
            Token::Substraction => write!(f, " - "),
            Token::Multiplication => write!(f, " * "),
            Token::Division => write!(f, " / "),
            Token::AntiSlash => write!(f, r" \ "),
            Token::Bang => write!(f, " ! "),
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
            Token::Literal(s) => write!(f,"\"{}\"",s),
            Token::VarIdentifier(i) => write!(f,"{}",i),
            Token::Char(c) => write!(f,"'{}",c),
            Token::Number(u) => write!(f,"{}",u),
            Token::InputChar(c) => write!(f,"?'{}",c),
            Token::InputNumber(u) => write!(f,"?{}",u),


        }
    }
}
