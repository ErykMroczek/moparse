use crate::{syntax::SyntaxKind, Token};

#[derive(Debug, PartialEq)]
/// Stores info about the `Enter` or `Exit` syntax events
pub struct Payload {
    /// Modelica production, like `equation`
    pub typ: SyntaxKind,
    /// index of the corresponding token from the `TokenCollection`
    pub tok: usize,
    /// index of this event
    pub idx: usize,
    /// index of the corresponding opposite event in the events list
    pub pair: usize,
}

#[derive(Debug, PartialEq)]
/// Represents a single Modelica syntax event.
///
/// Syntax event may mark starts and ends of productions or terminals.
/// The list of such syntax events should be consumed to build a parse
/// tree or an AST.
pub enum SyntaxEvent {
    /// Event indicating beginning of the Modelica production.
    Enter(Payload),
    /// Event indicating an end of some Modelica production.
    Exit(Payload),
    /// Event indicating a terminal (token or error).
    Advance(Terminal),
}

impl<'a> SyntaxEvent {
    /// Return a reference to the token from the `TokenCollection`
    /// corresponds to this event.
    pub fn get_token(&'a self, tokens: &'a Vec<Token>) -> &'a Token {
        match self {
            Self::Enter(p) | Self::Exit(p) => tokens.get(p.tok).unwrap(),
            Self::Advance(term) => term.get_token(tokens),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Represents a terminal (token).
///
/// Token may be pointed directly (`Token` variant) or wrapped together
/// with the error message (`Error` variant).
pub enum Terminal {
    /// Represents a token that caused the syntax error.
    ///
    /// * `msg`: error message
    /// * `tok`: index of the corresponding token from the
    ///   `TokenCollection`
    Error { msg: String, tok: usize },
    /// Represents normal (expected) token. Contains an index of the
    /// corresponding token from the `TokenCollection`
    Token(usize),
}

impl<'a> Terminal {
    /// Return a reference to the token from the `TokenCollection`
    /// corresponds to this event.
    pub fn get_token(&'a self, tokens: &'a Vec<Token>) -> &'a Token {
        match self {
            Self::Token(i) => tokens.get(*i).unwrap(),
            Self::Error { tok, .. } => tokens.get(*tok).unwrap(),
        }
    }
}
