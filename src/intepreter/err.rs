use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RuntimeError {
    FunctionNotDefined(String),
    // Tape Errors
    TapeIndexOutOfBounds(usize, usize),
    TapeIndexCannotBeNegative,

    // Stack Errors
    StackIndexCannotBeNegative,
    StackIndexOutOfBounds(usize, usize),
    Generic,
}

// Custom Dispaly for warnings
impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\nERROR: ")?;
        match self {
            RuntimeError::TapeIndexOutOfBounds(size, len) => f.write_fmt(format_args!(
                "Tape index {} out of bounds of tape size {}",
                size, len
            )),
            RuntimeError::StackIndexOutOfBounds(size, len) => f.write_fmt(format_args!(
                "Tape index {} out of bounds of stack size {}",
                size, len
            )),

            _ => f.write_fmt(format_args!("{:?}", self)),
        }
    }
}
