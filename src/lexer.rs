
#[derive(Debug,Clone)]
pub struct Lexer {
    pub source_file: String,
    pub tokens : Vec<Token>,
}
impl Lexer {
    pub fn new(input:&str)->Self {
       Self { source_file : input.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn chop_token(&mut self)-> Self {
        let bindings = self.source_file.split_whitespace().collect::<String>();
        let mut all_tokens = bindings.chars();
        loop {
            let  a_token= all_tokens.next();
            if a_token.is_none(){ break;
            } else {
                match a_token.unwrap() {
                  '$'  => {self.tokens.push(Token::Symbole(Symbol::DollarSign))  ; continue;},
                  '+'  => {self.tokens.push(Token::Symbole(Symbol::Addition) ) ; continue;},
                  '-'  => {self.tokens.push(Token::Symbole(Symbol::Substraction))      ; continue;},
                  '*'  => {self.tokens.push(Token::Symbole(Symbol::Multiplication))    ; continue;},
                  '/'  => {self.tokens.push(Token::Symbole(Symbol::Division)  )        ; continue;},
                  '\\' => {self.tokens.push(Token::Symbole(Symbol::AntiSlash))   ; continue;},
                  '?'  => {self.tokens.push(Token::Symbole(Symbol::QuestionMark)) ; continue;},
                  '!' => {self.tokens.push(Token::Symbole(Symbol::Bang)) ; continue;},
                  '"'  => {self.tokens.push(Token::Symbole(Symbol::QuotationMark))  ; continue;},
                  '\''  => {self.tokens.push(Token::Symbole(Symbol::Apostrophe))  ; continue;},
                  ':'  => {self.tokens.push(Token::Symbole(Symbol::Colon)) ; continue;},
                  '.'  => {self.tokens.push(Token::Symbole(Symbol::Point)) ; continue;},
                  '<'  => {self.tokens.push(Token::Symbole(Symbol::LessThan)) ; continue;},
                  '='  => {self.tokens.push(Token::Symbole(Symbol::Equal)); continue;},
                  '>'  => {self.tokens.push(Token::Symbole(Symbol::GreaterThan))       ; continue;},
                  '['  => {self.tokens.push(Token::Symbole(Symbol::LeftSquareBracket)) ; continue;},
                  ']'  => {self.tokens.push(Token::Symbole(Symbol::RightSquareBracket)); continue;},
                  '('  => {self.tokens.push(Token::Symbole(Symbol::LeftParnathesis) )  ; continue;},
                  ')'  => {self.tokens.push(Token::Symbole(Symbol::RightParnathesis) ) ; continue;},
                  '^'  => {self.tokens.push(Token::Symbole(Symbol::Caret)          )   ; continue;},
                  '#'  => {self.tokens.push(Token::Symbole(Symbol::Pound)           )  ; continue;},
                  '@'  => {self.tokens.push(Token::Symbole(Symbol::AtSign)           ) ; continue;},
                  '%'  => {self.tokens.push(Token::Symbole(Symbol::Ampersand)        ) ; continue;},
                  ','  => {self.tokens.push(Token::Symbole(Symbol::Comma)           )  ; continue;},
                  ';'  => {self.tokens.push(Token::Symbole(Symbol::SemiColon)        ) ; continue;},
                  '{'  => {self.tokens.push(Token::Symbole(Symbol::LeftBracket)      ) ; continue;},
                  '}'  => {self.tokens.push(Token::Symbole(Symbol::RightBracket)     ) ; continue;},
                  '~'  => {self.tokens.push(Token::Symbole(Symbol::Tilde)            ) ; continue;},
                  'a'..='z'| 'A'..='Z' => {
                        self.tokens.push(Token::Charcter(a_token.unwrap()));
                        continue;
                      },
                    _ => { self.tokens.push(Token::Identifier(a_token.unwrap()));
                          continue;},
                };
}
            }
         Self { source_file:self.source_file.clone(),
            tokens: self.tokens.clone()
           }
        }
}

#[derive(Debug,Clone)]
pub enum Token {
    Symbole (Symbol),
    Charcter(char),
    Identifier(char)

}

#[derive(Debug,Clone)]
 pub enum Symbol {
    DollarSign,    	     //         $
    Addition,            //         +
    Substraction,   	 //         -
    Multiplication,      //         *
    Division,            //         /
    AntiSlash,           //         \
    QuestionMark,        //         ?
    Bang,                //
    Apostrophe,          //         '
    QuotationMark,       //         "
    Colon,               //         :
    Point,               //         .
    LessThan,            //         <
    Equal,               //         =
    GreaterThan,         //         >
    LeftSquareBracket,   //         [
    RightSquareBracket,  //         ]
    LeftParnathesis,     //         (
    RightParnathesis,    //         )
    Caret,               //         ^
    Pound,               //         #
    AtSign,              //         @
    Ampersand,           //         %
    Comma,               //         ,
    SemiColon,           //         ;
    LeftBracket,         //         {
    RightBracket,        //         }
    Tilde,               //         ~

}
