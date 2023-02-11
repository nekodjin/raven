#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Error {
    pub kind: Kind,
    pub span: super::Span,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    InvalidUtf8,
    UnexpectedEoi,

    InvalidRawIdent,
}
