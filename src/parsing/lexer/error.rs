#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub kind: Kind,
    pub span: super::Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    InvalidUtf8,
    UnexpectedEoi,
}
