use moparse::{events, lex, SyntaxEvent, SyntaxKind};
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
    let (tokens, comments, mut errors) = lex(&contents);
    let (events, mut p_errors) = events(&tokens, SyntaxKind::DerClassSpecifier);
    errors.append(&mut p_errors);
    if errors.len() > 0 {
        let msg = errors.iter().map(|e| format!("{}:{}", file_path, e)).collect::<Vec<_>>().join("\n");
        panic!("Syntax errors detected:\n{}", msg);
        
    }
    let mut tokens = tokens.into_iter();
    let out = events.iter().map(|e| match e {
        SyntaxEvent::Advance => format!("{:?}", tokens.next().unwrap()),
        SyntaxEvent::Enter(p) => format!("{:?}", p),
        SyntaxEvent::Exit => String::new(),
    }).collect::<Vec<_>>().join("\n");
    println!("{}", out);
}
