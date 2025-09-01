use std::{env, fs, process};

mod lexparse;
use lexparse::lexer::Lexer;

fn main() -> Result<(), ()> {
    let args = env::args().collect::<Vec<String>>();
    if args.is_empty() {
        eprintln!("no file was provided");
        eprintln!("Usage: Command <File-Name>");
        process::exit(1);
    }
    let file_content = fs::read_to_string(&args[1]).unwrap();
    //println!("opening {file_name} for execution",file_name=&args[1]);
    // let lexer = Lexer::new(&file_content).chop_token();
    // for token in lexer.tokens {
    //    println!("{token:?}");
    //}
    let mut lexer = Lexer::new(&file_content);

    let tokens = lexer.lex();
    for token in tokens {
        match token {
            Ok(t) => println!("{t}"),
            Err(_) => println!("error occured "),
        }
    }

    Ok(())
}
