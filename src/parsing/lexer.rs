use std::io::Read;

use error::*;

use super::token::*;

mod error;

type Result<T, E = Error> = ::std::result::Result<T, E>;

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
    pub fn next_token(&mut self) -> Result<Token> {
        if let Some(token) = self._nx_tok.take() {
            return Ok(token);
        }

        self._nx_tok()
    }

    // TODO: remove `clone` use once Token impls Copy
    pub fn peek_token(&mut self) -> Result<Token> {
        if let Some(token) = self._nx_tok.clone() {
            return Ok(token);
        }

        let token = self._nx_tok()?;

        self._nx_tok = Some(token.clone());

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

        if fst_char == '_' || fst_char.is_ascii_alphabetic() {
            let mut buf = String::from(fst_char);

            while self
                .peek_char()
                .map(|c| c == '_' || c.is_ascii_alphanumeric())
                .unwrap_or(false)
            {
                buf.push(
                    self.next_char()
                        .expect("no next char after peek_char was Ok(..)"),
                );
            }

            return Ok(Token {
                span: Span::new(start, self.pos),
                data: TokenData::Ident(buf),
            });
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

            Err(invalid_utf8_err)
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

        let str = String::from_utf8(bytes).map_err(|_| invalid_utf8_err)?;

        if str.chars().count() != 1 {
            panic!("consumed codepoint {:?} incorrectly @{}", save, self.pos);
        }

        Ok(str.chars().next().expect("non-empty str had no first char"))
    }
}
