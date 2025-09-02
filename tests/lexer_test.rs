use msl_interpreter::lexparse::*;

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
fn lex_symbol_literal() {
    let sourcefile = format!("\"This is a string literal\"");
    let start = sourcefile.chars().next().unwrap().len_utf8();
    let end = sourcefile[start + 1..].find('"').unwrap();
    let end2 = sourcefile[end..].find('"').unwrap();
    let literal = &sourcefile[start..end + end2];

    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Literal(literal));
    }
}

#[test]
fn lex_symbol_apostrophe() {
    let sourcefile = String::from("\'");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Apostrophe);
    }
}

#[test]
fn lex_symbol_question_mark() {
    let sourcefile = String::from("?");
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::QuestionMark);
    }
}

#[test]
fn lex_symbol_number() {
    let sourcefile = String::from("1235");
    let num = sourcefile.parse().unwrap();
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Number(num));
    }
}

#[test]
fn lex_symbol_char() {
    let sourcefile = String::from('a');
    let car = sourcefile.parse().unwrap();
    let mut lexer = lexer::Lexer::new(&sourcefile);
    let tokens = lexer.lex();
    if let Ok(token) = tokens.first().unwrap() {
        assert_eq!(token.token, lexer::Token::Char(car));
    }
}
