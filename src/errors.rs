use std::fmt::{Display, Formatter, Error};


pub struct SyntaxError {
    msg: String,
    line: usize,
    col: usize,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}:{}: {}", self.line, self.col, self.msg)
    }
}
