use crate::{syntax::SyntaxKind, Token, TokenCollection};

#[derive(Debug, PartialEq)]
/// Represents a single Modelica syntax event.
///
/// Syntax event may mark starts and ends of productions or terminals.
/// The list of such syntax events should be consumed to build a parse
/// tree or an AST.
pub enum SyntaxEvent {
    /// Event indicating beginning of the Modelica production.
    ///
    /// * `typ`: Modelica production, like `equation`
    /// * `tok`: index of the corresponding token from the
    ///   `TokenCollection`
    /// * `exit`: index of the corresponding `Exit` event in the events
    ///   list
    Enter {
        typ: SyntaxKind,
        tok: usize,
        exit: usize,
    },
    /// Event indicating an end of some Modelica production.
    ///
    /// * `typ`: Modelica production, like `equation`
    /// * `tok`: index of the corresponding token from the
    ///   `TokenCollection`
    /// * `enter`: index of the `Enter` event that corresponds to this
    ///   `Exit`
    Exit {
        typ: SyntaxKind,
        tok: usize,
        enter: usize,
    },
    /// Event indicating a terminal (token or error).
    ///
    /// Contains an instance of the `Terminal` enum.
    Advance(Terminal),
}

impl<'a> SyntaxEvent {
    /// Return a reference to the token from the `TokenCollection`
    /// corresponds to this event.
    pub fn get_token(&'a self, tokens: &'a TokenCollection) -> &'a Token {
        match self {
            Self::Enter { tok, .. } | Self::Exit { tok, .. } => tokens.get_token(*tok).unwrap(),
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
    pub fn get_token(&'a self, tokens: &'a TokenCollection) -> &'a Token {
        match self {
            Self::Token(i) => tokens.get_token(*i).unwrap(),
            Self::Error { tok, .. } => tokens.get_token(*tok).unwrap(),
        }
    }
}
