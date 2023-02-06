#[derive(Debug, PartialEq)]
pub struct Token {
    data: TokenData,
    span: Span,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Span {
    pub start: Location,
    pub end: Option<Location>,
}

impl Span {
    pub const fn point(point: Location) -> Self {
        Span { start: point, end: None }
    }

    pub const fn span(start: Location, end: Location) -> Self {
        Span { start, end: Some(end) }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Location {
    pub lin: u32,
    pub col: u32,
}

impl Location {
    pub const fn location(lin: u32, col: u32) -> Self {
        Location { lin, col }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenData {
    Ident(String),
}
