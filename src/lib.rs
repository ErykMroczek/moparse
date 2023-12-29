// Public elements
pub use self::tokens::{TokenKind, Token, TokenCollection};
pub use self::syntax::SyntaxKind;
pub use self::events::{SyntaxEvent, Terminal};
pub use self::lexing::lex;
pub use self::parsing::parse;

// Private elements
mod syntax;
mod lexing;
mod tokens;
mod events;
mod parsing;
