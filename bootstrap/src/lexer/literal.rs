use crate::core::span::TextSpan;
use crate::core::token::{Literal, Token, TokenKind};
use crate::core::token::Literal::{False, Number, True};
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_string(&self, c: char) -> bool {
        c == '\''
    }
    pub(crate) fn consume_string(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let mut text = String::from("");
        let next = String::from(self.consume_next()?);
        assert_eq!(next, "\'");

        loop {
            let next = self.consume_next()?;
            if next == '\\' {
                if self.peek_if("'").is_some() {
                    let _ = self.consume_next()?;
                    text.push_str("'");
                    continue;
                }
            }
            if next == '\'' {
                break;
            }

            text.push(next);
        }

        Ok(Token {
            kind: TokenKind::Literal(Literal::String),
            span: TextSpan { start, end: self.position(), value: text },
        })
    }

    pub(crate) fn is_number(&self, c: char) -> bool {
        c.is_digit(10)
    }

    pub(crate) fn consume_number(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let mut text = String::from("");
        let next = String::from(self.consume_next()?);

        if next == "0" {
            if let Some(c) = self.peek_next() {
                match c {
                    'x' | 'X' => {
                        self.consume_next()?;
                        text.push_str("0x");
                        text.push_str(&*self.consume_while(|c| c.is_digit(16))?);
                    }
                    'o' | 'O' => {
                        self.consume_next()?;
                        text.push_str("0o");
                        text.push_str(&*self.consume_while(|c| c.is_digit(8))?);
                    }
                    'b' | 'B' => {
                        self.consume_next()?;
                        text.push_str("0b");
                        text.push_str(&*self.consume_while(|c| c.is_digit(2))?);
                    }
                    c if c.is_digit(10) => {
                        text.push_str(&*self.consume_while(|c| c.is_digit(10))?);
                    }
                    _ => {
                        text.push_str("0");
                    }
                }
            }
        } else {
            text.push_str(&next);
            let next = self.consume_while(|c| c.is_digit(10))?;
            text.push_str(&next);

            if let Some('.') = self.peek_next() {
                let _ = self.consume_next()?;
                text.push('.');
                let next = self.consume_while(|c| c.is_digit(10))?;
                text.push_str(&next);
            }
        }

        Ok(Token {
            kind: TokenKind::Literal(Number),
            span: TextSpan { start, end: self.position(), value: text },
        })
    }

    pub(crate) fn is_bool(&self, c: char) -> bool {
        if c != 't' && c != 'f' {
            return false
        }
        let look_ahead = self.look_ahead().unwrap();
        return matches!(look_ahead.as_str(), "true" | "false");
    }

    pub(crate) fn consume_bool(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let next = self.consume_next()?;
        if next == 't' {
            self.consume_if("rue").unwrap();
            return Ok(Token {
                kind: TokenKind::Literal(True),
                span: TextSpan { start, end: self.position(), value: String::from("true") },
            });
        }

        assert_eq!(next, 'f');
        self.consume_if("alse").unwrap();
        Ok(Token {
            kind: TokenKind::Literal(False),
            span: TextSpan { start, end: self.position(), value: String::from("false") },
        })
    }
}


#[cfg(test)]
mod test {
    use crate::core::token::Literal::{False, Number, String, True};
    use crate::core::token::Operator::*;
    use crate::core::token::TokenKind;
    use crate::lexer::Lexer;

    #[test]
    fn empty_string() {
        let text = "''";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.value, "");
    }

    #[test]
    fn hello_elodie() {
        let text = "'Hello Elodie'";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 15, 14));
        assert_eq!(result.span.value, "Hello Elodie");
    }

    #[test]
    fn escaped_string() {
        let text = "'{\\'hello\\':\\'world\\'}'";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(String));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 24, 23));
        assert_eq!(result.span.value, "{'hello':'world'}");
    }

    #[test]
    fn nat() {
        let text = "42";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.value, "42");
    }


    #[test]
    fn float() {
        let text = "42.24";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.value, "42.24");
    }

    #[test]
    fn hex() {
        let text = "0xDEADBEEF";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 11, 10));
        assert_eq!(result.span.value, "0xDEADBEEF");
    }

    #[test]
    fn octal() {
        let text = "0o10";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.value, "0o10");
    }

    #[test]
    fn binary() {
        let text = "0b10101";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Number));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.value, "0b10101");
    }

    #[test]
    fn r#true() {
        let text = "true";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(True));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 5, 4));
        assert_eq!(result.span.value, "true");
    }

    #[test]
    fn not_true() {
        let text = "true_";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.value, "true_");
    }

    #[test]
    fn r#false() {
        let text = "false";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(False));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.value, "false");
    }

    #[test]
    fn not_false() {
        let text = "false_";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 7, 6));
        assert_eq!(result.span.value, "false_");
    }

}