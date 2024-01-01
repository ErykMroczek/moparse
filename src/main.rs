use moparse::{parse, lex, SyntaxEvent, Terminal};
use moparse::SyntaxKind;
use std::fs;
use std::env;

// TODO: Provide executable that would generate JSON object from the Modelica file
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
        panic!("Only one argument (path to the Modelica file) is allowed")
    }
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("error reading a file");
    let tokens = lex(&contents);
    let events = parse(&tokens, SyntaxKind::StoredDefinition);
    for event in events {
        match event {
            SyntaxEvent::Advance(t) => {
                match t {
                    Terminal::Token(i) => println!("{:?}", tokens.get_token(i).unwrap()),
                    Terminal::Error { msg, tok} => println!("Error: '{}' ({:?})", msg, tokens.get_token(tok).unwrap()),
                }
            },
            SyntaxEvent::Enter(p) => println!("Enter: {:?}", p.typ),
            SyntaxEvent::Exit(p) => println!("Exit: {:?}", p.typ),
        }
    }
}
