use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Copy, Clone)]
/// Represents a type of a Modelica token. Defined based on [Modelica
/// Specification
/// 3.6](https://specification.modelica.org/maint/3.6/modelica-concrete-syntax.html).
pub enum ModelicaToken {
    Comma,
    Dot,
    Semicolon,
    Colon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    Equal,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Flex,
    DotPlus,
    DotMinus,
    DotStar,
    DotSlash,
    DotFlex,
    Gre,
    Geq,
    Les,
    Leq,
    Neq,
    Eq,
    Not,
    And,
    Or,
    In,
    For,
    If,
    Else,
    ElseIf,
    Then,
    When,
    ElseWhen,
    While,
    Loop,
    Break,
    Return,
    Partial,
    Class,
    Operator,
    Expandable,
    Model,
    Function,
    Record,
    Type,
    Block,
    Connector,
    Package,
    Pure,
    Impure,
    Initial,
    Equation,
    Algorithm,
    Extends,
    Import,
    Public,
    Protected,
    Within,
    Final,
    Encapsulated,
    Enumeration,
    Input,
    Output,
    Redeclare,
    Inner,
    Outer,
    Replaceable,
    Constrainedby,
    Flow,
    Stream,
    Discrete,
    Parameter,
    Constant,
    Each,
    Annotation,
    External,
    End,
    Der,
    Connect,
    LineComment,
    BlockComment,
    Identifier,
    String,
    UInt,
    UReal,
    True,
    False,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Used to represent token's position in the input string
///
/// * `pos`: index of the character that corresponds with this position
/// * `line`: line number of the character that corresponds with this
///   position
/// * `col`: column number of the character that corresponds with this
///   position
pub struct Position {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
/// Represents a single Modelica token.
///
/// Tokens contain information on their type and their coordinates in
/// the source.
///
/// * `idx`: position in the token collection
/// * `text`: text content of the token
/// * `kind`: token's kind
/// * `start`: position of the first character
/// * `end`: position of the last character
pub struct Token {
    /// Index of the token in the input
    pub idx: usize,
    /// Text of the token
    pub text: String,
    /// Token's kind
    pub kind: ModelicaToken,
    /// Position of staring character in the input
    pub start: Position,
    /// Positon of ending character in the input
    pub end: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "'{}'", self.text)
    }
}

impl Token {

}
