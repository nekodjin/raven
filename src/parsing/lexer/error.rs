use crate::interning::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Error {
    pub kind: Kind,
    pub span: super::Span,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    /// Represents an error parsing the source code as utf-8
    InvalidUtf8,
    /// Represents unexpected termination of the input stream
    UnexpectedEoi,

    /// Represents the # raw ident prefix being followed by an invalid
    /// character.
    InvalidRawIdent,
    /// Represents an int literal prefix such as `0b` or `0x` being followed by
    /// a non-digit character.
    EmptyPrefixedInt,
    /// Represents a prefixed int literal such as `0b123` or `0o69` which is
    /// followed by valid digit characters, but whose digits are invalid for
    /// the specified radix.
    InvalidPrefixedInt {
        /// The radix specified by the literal's prefix
        radix: u32,
    },
}
