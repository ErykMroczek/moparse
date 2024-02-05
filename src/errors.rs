use std::fmt::{Display, Error, Formatter};

pub struct SyntaxError {
    pub msg: String,
    pub line: usize,
    pub col: usize,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}:{}: {}", self.line, self.col, self.msg)
    }
}

impl SyntaxError {
    pub fn new(msg: String, line: usize, col: usize) -> SyntaxError {
        SyntaxError { msg, line, col }
    }
}
