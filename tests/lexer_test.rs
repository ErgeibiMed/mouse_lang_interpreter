use mouse_lang_interpreter::lexparse::*;

#[test]
fn lex_whitspace() {
    let sourcefile = String::from(" ");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Whitespace);
    }
}

#[test]
fn lex_dollar_sign() {
    let sourcefile = String::from("$");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::DollarSign);
    }
}

#[test]
fn lex_addition() {
    let sourcefile = String::from("+");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Addition);
    }
}
#[test]
fn lex_symbol_substraction() {
    let sourcefile = String::from("-");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Substraction);
    }
}
#[test]
fn lex_symbol_multiplication() {
    let sourcefile = String::from("*");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Multiplication);
    }
}
#[test]
fn lex_symbol_division() {
    let sourcefile = String::from("/");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Division);
    }
}
#[test]
fn lex_symbol_anti_slash() {
    let sourcefile = String::from(r"\");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::AntiSlash);
    }
}
#[test]
fn lex_symbol_bang() {
    let sourcefile = String::from("!");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Bang);
    }
}

#[test]
fn lex_symbol_colon() {
    let sourcefile = String::from(':');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Colon);
    }
}
#[test]
fn lex_symbol_point() {
    let sourcefile = String::from('.');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Point);
    }
}

#[test]
fn lex_symbol_less_than() {
    let sourcefile = String::from('<');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::LessThan);
    }
}
#[test]
fn lex_symbol_equal() {
    let sourcefile = String::from('=');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Equal);
    }
}
#[test]
fn lex_symbol_greater_than() {
    let sourcefile = String::from('>');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::GreaterThan);
    }
}
#[test]
fn lex_symbol_left_square_bracket() {
    let sourcefile = String::from('[');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::LeftSquareBracket);
    }
}
#[test]
fn lex_symbol_right_square_bracket() {
    let sourcefile = String::from(']');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::RightSquareBracket);
    }
}
#[test]
fn lex_symbol_left_parnathesis() {
    let sourcefile = String::from('(');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::LeftParnathesis);
    }
}
#[test]
fn lex_symbol_right_parnathesis() {
    let sourcefile = String::from(')');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::RightParnathesis);
    }
}
#[test]
fn lex_symbol_caret() {
    let sourcefile = String::from('^');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Caret);
    }
}
#[test]
fn lex_symbol_pound() {
    let sourcefile = String::from('#');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Pound);
    }
}
#[test]
fn lex_symbol_at_sign() {
    let sourcefile = String::from('@');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::AtSign);
    }
}
#[test]
fn lex_symbol_ampersand() {
    let sourcefile = String::from('%');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Ampersand);
    }
}
#[test]
fn lex_symbol_comma() {
    let sourcefile = String::from(',');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Comma);
    }
}
#[test]
fn lex_symbol_semi_colon() {
    let sourcefile = String::from(';');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::SemiColon);
    }
}
#[test]
fn lex_symbol_left_bracket() {
    let sourcefile = String::from('{');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::LeftBracket);
    }
}

#[test]
fn lex_symbol_right_bracket() {
    let sourcefile = String::from('}');
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::RightBracket);
    }
}
//
#[test]
fn lex_symbol_tilde() {
    let sourcefile = String::from("~  This is a comment\n");
    let start = sourcefile.chars().next().unwrap();
    let endline = sourcefile.find('\n').unwrap(); //Comment always strat with tilde and end at the newline
    let comment = &sourcefile[start.len_utf8() + 1..endline];
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Tilde(&comment));
    }
}
#[test]
fn lex_symbol_var_identifier() {
    let sourcefile = String::from("kfa55d41d7*g");

    let chars = sourcefile.chars();
    let mut whole: Vec<char> = Vec::new();
    let rst = chars
        .take_while(|v| v.is_digit(10) || v.is_alphabetic())
        .collect::<Vec<char>>();
    for i in 0..rst.len() {
        whole.push(rst[i]);
    }
    let start = 0;
    let end_byte = start + whole.len();
    let var_ident = &sourcefile[start..end_byte];
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::VarIdentifier(var_ident));
    }
}
////continue from here
//#[test]
//fn lex_symbol_literal() {
//    let sourcefile = format!("\"This is a string literal\"");
//    let start = 0;
//    let end = sourcefile[start + 1..].find('"').unwrap();
//    let literal = &sourcefile[start..end + 1];
//
//    let mut lexer = lexer::Lexer::new(&sourcefile);
//    let tokens = lexer.lex();
//    if let Ok(token) = tokens.first().unwrap() {
//        assert_eq!(token.token, lexer::Token::Literal(literal));
//    }
//}
//#[test]
//fn lex_symbol_Char(){
//let sourcefile=String::from('Char');
//let mut lexer = lexer::Lexer::new(&sourcefile);
//let tokens=lexer.lex();
//if let Ok(token)=tokens.first().unwrap() {
//assert_eq!(token.token,lexer::Token::Char);
//}}
//#[test]
//fn lex_symbol_Number(){
//let sourcefile=String::from('Number');
//let mut lexer = lexer::Lexer::new(&sourcefile);
//let tokens=lexer.lex();
//if let Ok(token)=tokens.first().unwrap() {
//assert_eq!(token.token,lexer::Token::Number);
//}}
//#[test]
//fn lex_symbol_InputChar(){
//let sourcefile=String::from('InputChar');
//let mut lexer = lexer::Lexer::new(&sourcefile);
//let tokens=lexer.lex();
//if let Ok(token)=tokens.first().unwrap() {
//assert_eq!(token.token,lexer::Token::InputChar);
//}}
//#[test]
//fn lex_symbol_InputNumber(){
//let sourcefile=String::from('InputNumber');
//let mut lexer = lexer::Lexer::new(&sourcefile);
//let tokens=lexer.lex();
//if let Ok(token)=tokens.first().unwrap() {
//assert_eq!(token.token,lexer::Token::InputNumber);
//}}
