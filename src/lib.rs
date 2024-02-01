use lexing::lex;
use parsing::events;
// Public elements
pub use self::events::SyntaxEvent;
pub use self::syntax::SyntaxKind;
pub use self::tokens::{ModelicaToken, Token};
pub use errors::SyntaxError;

/// Output from the parser.
/// Contains everything necesary to build a parse tree.
pub struct ParsedModelica {
    pub tokens: Vec<Token>,
    pub comments: Vec<Token>,
    pub events: Vec<SyntaxEvent>,
    pub errors: Vec<SyntaxError>,
}

/// Return `Parsed` object generated from the `source` string.
pub fn parse(source: &str, entry: SyntaxKind) -> ParsedModelica {
    let (tokens, comments, mut errors) = lex(source);
    let (events, mut p_errors) = events(&tokens, entry);
    errors.append(&mut p_errors);
    ParsedModelica {
        tokens,
        comments,
        events,
        errors,
    }
}

// Private elements
mod errors;
mod events;
mod lexing;
mod parsing;
mod syntax;
mod tokens;
