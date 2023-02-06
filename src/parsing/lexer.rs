use super::token::*;
use error::*;
use std::io::Read;

mod error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub struct Lexer<Source>
where
    Source: Read,
{
    source: Source,
    loc: Location,
    _nx_tok: Token,
    _nx_chr: u8,
}

impl<Source> Lexer<Source>
where
    Source: Read,
{
    fn _nx_chr(&mut self) -> Result<char> {
        let mut get_byte = || {
            let mut buffer = [0];

            let Err(err) = self.source.read_exact(&mut buffer) else {
                return Ok(buffer[0]);
            };

            if err.kind() != std::io::ErrorKind::UnexpectedEof {
                panic!("unexpected error: {err}");
            }

            Err(Error {
                kind: Kind::UnexpectedEoi,
                span: Span::point(self.loc),
            })
        };

        let byte = get_byte()?;
        let len =

        if byte & 0b1000_0000 == 0b0000_0000 {
            1
        }
        else if byte & 0b1110_0000 == 0b1100_0000 {
            2
        }
        else if byte & 0b1111_0000 == 0b1110_0000 {
            3
        }
        else {
            return Err(Error {
                kind: Kind::InvalidUtf8,
                span: Span::point(self.loc),
            });
        };

        todo!();
    }
}
