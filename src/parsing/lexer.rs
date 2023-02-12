use std::io::Read;

use error::*;
use num::Num;

use super::token::*;
use crate::interning::*;

mod error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Lexer<Source>
where
    Source: Read,
{
    source:  Source,
    pos:     Position,
    _nx_tok: Option<Token>,
    _nx_chr: Option<char>,
}

impl<Source> Lexer<Source>
where
    Source: Read,
{
    pub fn new(source: Source) -> Self {
        Lexer {
            source,
            pos: Position::new(1, 0),
            _nx_tok: None,
            _nx_chr: None,
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if let Some(token) = self._nx_tok.take() {
            return Ok(token);
        }

        self._nx_tok()
    }

    pub fn peek_token(&mut self) -> Result<Token> {
        if let Some(token) = self._nx_tok {
            return Ok(token);
        }

        let token = self._nx_tok()?;

        self._nx_tok = Some(token);

        Ok(token)
    }

    fn _nx_tok(&mut self) -> Result<Token> {
        let fst_char = loop {
            match self.peek_char()? {
                ' ' | '\t' | '\r' => continue,
                char => break char,
            }
        };

        let start = self.pos;

        if fst_char == '(' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::LParen,
            });
        }

        if fst_char == ')' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::RParen,
            });
        }

        if fst_char == '[' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::LBrack,
            });
        }

        if fst_char == ']' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::RBrack,
            });
        }

        if fst_char == '{' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::LBrace,
            });
        }

        if fst_char == '}' {
            return Ok(Token {
                span: Span::point(start),
                data: TokenData::RBrace,
            });
        }

        let lex_ident = |lexer: &mut Lexer<Source>, char, raw| {
            let mut buf = StdString::from(char);

            while lexer
                .peek_char()
                .map(|c| c == '_' || c.is_ascii_alphanumeric())
                .unwrap_or(false)
            {
                buf.push(
                    lexer
                        .next_char()
                        .expect("no next char after peek_char was Ok(..)"),
                );
            }

            let val = &*buf;

            let data = if raw {
                TokenData::Ident {
                    raw,
                    val: String::from(val),
                }
            }
            else {
                use TokenData::*;

                match val {
                    "true" => KwTrue,
                    "false" => KwFalse,
                    _ => {
                        Ident {
                            raw,
                            val: String::from(val),
                        }
                    },
                }
            };

            Ok(Token {
                span: Span::new(start, lexer.pos),
                data,
            })
        };

        if fst_char == '#' {
            let next_char = self.next_char()?;

            if next_char != '_' && !next_char.is_ascii_alphabetic() {
                return Err(Error {
                    kind: Kind::InvalidRawIdent,
                    span: Span::new(start, self.pos),
                });
            }

            return lex_ident(self, next_char, true);
        }

        if fst_char == '_' || fst_char.is_ascii_alphabetic() {
            return lex_ident(self, fst_char, false);
        }

        let lex_prefixed_int = |lexer: &mut Lexer<Source>, radix| {
            let mut buf = StdString::new();

            while lexer
                .peek_char()
                .map(|c| c.is_ascii_hexdigit())
                .unwrap_or(false)
            {
                buf.push(
                    lexer
                        .next_char()
                        .expect("no next char after peek_char was Ok(..)"),
                );
            }

            let buf = buf;

            if buf.is_empty() {
                return Err(Error {
                    kind: Kind::EmptyPrefixedInt,
                    span: Span::new(start, lexer.pos),
                });
            }

            match radix {
                2 | 8 | 10 | 16 => {},
                _ => panic!("invalid radix passed: {radix}"),
            }

            StdBigInt::from_str_radix(&buf, radix)
                .map(|val| {
                    Token {
                        span: Span::new(start, lexer.pos),
                        data: TokenData::IntLit(BigInt::from(val)),
                    }
                })
                .map_err(|_| {
                    Error {
                        kind: Kind::InvalidPrefixedInt { radix },
                        span: Span::new(start, lexer.pos),
                    }
                })
        };

        let lex_decimal_int = |lexer: &mut Lexer<Source>, fst_char| {
            let mut buf = StdString::from(fst_char);

            while lexer
                .peek_char()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
            {
                buf.push(
                    lexer
                        .next_char()
                        .expect("no next char after peek_char was Ok(..)"),
                );
            }

            let buf = buf;

            match buf.parse().map(|val: StdBigInt| {
                Token {
                    span: Span::new(start, lexer.pos),
                    data: TokenData::IntLit(BigInt::from(val)),
                }
            }) {
                Ok(token) => Ok(token),
                Err(_) => panic!("decimal literal failed to parse"),
            }
        };

        if fst_char == '0' {
            return match self.peek_char() {
                Ok('b') => lex_prefixed_int(self, 2),
                Ok('o') => lex_prefixed_int(self, 8),
                Ok('d') => lex_prefixed_int(self, 10),
                Ok('x') => lex_prefixed_int(self, 16),
                Ok(digit) if digit.is_ascii_digit() => {
                    lex_decimal_int(self, '0')
                },
                _ => {
                    Ok(Token {
                        span: Span::point(start),
                        data: TokenData::IntLit(BigInt::from(StdBigInt::from(
                            0,
                        ))),
                    })
                },
            };
        }

        if fst_char.is_ascii_digit() {
            return lex_decimal_int(self, fst_char);
        }

        todo!();
    }

    fn next_char(&mut self) -> Result<char> {
        if let Some(char) = self._nx_chr.take() {
            return Ok(char);
        };

        let char = self._nx_chr()?;

        match char {
            '\n' => {
                self.pos.lin += 1;
                self.pos.col = 1;
            },
            '\t' => self.pos.col += 4,
            _ => self.pos.col += 1,
        }

        Ok(char)
    }

    fn peek_char(&mut self) -> Result<char> {
        if let Some(char) = self._nx_chr {
            return Ok(char);
        }

        let char = self._nx_chr()?;

        self._nx_chr = Some(char);

        Ok(char)
    }

    fn _nx_chr(&mut self) -> Result<char> {
        let invalid_utf8_err = Error {
            kind: Kind::InvalidUtf8,
            span: Span::point(self.pos),
        };

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
                span: Span::point(self.pos),
            })
        };

        let fst_byte = get_byte()?;

        let len = if fst_byte & 0b1000_0000 == 0b0000_0000 {
            1
        }
        else if fst_byte & 0b1110_0000 == 0b1100_0000 {
            2
        }
        else if fst_byte & 0b1111_0000 == 0b1110_0000 {
            3
        }
        else if fst_byte & 0b1111_1000 == 0b1111_0000 {
            4
        }
        else if fst_byte & 0b1111_1100 == 0b1111_1000 {
            5
        }
        else if fst_byte & 0b1111_1110 == 0b1111_1100 {
            6
        }
        else {
            return Err(invalid_utf8_err);
        };

        let mut bytes = Vec::with_capacity(len);

        bytes.push(fst_byte);

        while bytes.len() < len {
            let byte = get_byte()?;

            if byte & 0b1100_0000 != 0b1000_0000 {
                return Err(invalid_utf8_err);
            }

            bytes.push(byte);
        }

        let save = bytes.clone();

        let str = StdString::from_utf8(bytes).map_err(|_| invalid_utf8_err)?;

        if str.chars().count() != 1 {
            panic!("consumed codepoint {:?} incorrectly @{}", save, self.pos);
        }

        Ok(str.chars().next().expect("non-empty str had no first char"))
    }
}

impl<Source> Iterator for Lexer<Source>
where
    Source: Read,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Err(Error {
                kind: Kind::UnexpectedEoi,
                ..
            }) => None,
            next => Some(next),
        }
    }
}
