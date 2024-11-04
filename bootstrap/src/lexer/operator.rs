use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::core::token::Operator::*;
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn is_operator(&self, c: char) -> bool {
        matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | '<' | '>' | '.' | ':' |
                 '+' | '-' | '*' | '/' | '&' | '|' | '^' | '%' | '~' | '=' | '!' | '?')
    }

    pub(crate) fn consume_operator(&self) -> crate::lexer::Result<Token> {
        let start = self.position();
        let mut text = String::from(self.consume_next()?);

        let kind = match text.as_str() {
            "(" => TokenKind::Operator(OpenParen),
            ")" => TokenKind::Operator(CloseParen),
            "{" => TokenKind::Operator(OpenCurly),
            "}" => TokenKind::Operator(CloseCurly),
            "[" => TokenKind::Operator(OpenBracket),
            "]" => TokenKind::Operator(CloseBracket),
            "<" => {
                match self.peek_next() {
                    Some('<') => {
                        let _ = self.consume_next()?;
                        text.push('<');
                        TokenKind::Operator(DoubleLeftAngle)
                    }
                    Some('=') => {
                        let _ = self.consume_next()?;
                        text.push('=');
                        TokenKind::Operator(LeftAngleEquals)
                    }
                    _ => TokenKind::Operator(LeftAngle)
                }
            }
            ">" => {
                match self.peek_next() {
                    Some('>') => {
                        let _ = self.consume_next()?;
                        text.push('>');
                        TokenKind::Operator(DoubleRightAngle)
                    }
                    Some('=') => {
                        let _ = self.consume_next()?;
                        text.push('=');
                        TokenKind::Operator(RightAngleEquals)
                    }
                    _ => TokenKind::Operator(RightAngle)
                }
            }
            "." => {
                match self.peek_next() {
                    Some('.') => {
                        let _ = self.consume_next()?;
                        text.push('.');
                        TokenKind::Operator(DoubleDot)
                    }
                    _ => TokenKind::Operator(Dot)
                }
            }
            ":" => {
                match self.peek_next() {
                    Some(':') => {
                        let _ = self.consume_next()?;
                        text.push(':');
                        TokenKind::Operator(DoubleColon)
                    }
                    _ => TokenKind::Operator(Colon)
                }
            }
            "-" => match self.peek_next() {
                Some('>') => {
                    let _ = self.consume_next()?;
                    text.push('>');
                    TokenKind::Operator(Arrow)
                }
                _ => TokenKind::Operator(Minus)
            }
            "+" => TokenKind::Operator(Plus),
            "*" => TokenKind::Operator(Asterisk),
            "/" => TokenKind::Operator(Slash),
            "&" => {
                match self.peek_next() {
                    Some('&') => {
                        let _ = self.consume_next()?;
                        text.push('&');
                        TokenKind::Operator(DoubleAmpersand)
                    }
                    _ => TokenKind::Operator(Ampersand)
                }
            }
            "|" => {
                match self.peek_next() {
                    Some('|') => {
                        let _ = self.consume_next()?;
                        text.push('|');
                        TokenKind::Operator(DoublePipe)
                    }
                    _ => TokenKind::Operator(Pipe)
                }
            }
            "^" => TokenKind::Operator(Caret),
            "%" => TokenKind::Operator(Percent),
            "=" => {
                match self.peek_next() {
                    Some('=') => {
                        let _ = self.consume_next()?;
                        text.push('=');
                        TokenKind::Operator(DoubleEquals)
                    }
                    _ => TokenKind::Operator(Equals)
                }
            }
            "!" => {
                match self.peek_next() {
                    Some('=') => {
                        let _ = self.consume_next()?;
                        text.push('=');
                        TokenKind::Operator(BangEquals)
                    }
                    _ => TokenKind::Operator(Bang)
                }
            }
            _ => return Err(crate::lexer::Error::UnknownOperator(text)),
        };

        Ok(Token {
            kind,
            span: TextSpan { start, end: self.position(), text },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::core::token::Operator::*;
    use crate::core::token::TokenKind;
    use crate::lexer::Lexer;

    #[test]
    fn open_paren() {
        let text = "(";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OpenParen));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "(");
    }

    #[test]
    fn close_paren() {
        let text = ")";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(CloseParen));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ")");
    }

    #[test]
    fn open_curly() {
        let text = "{";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OpenCurly));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "{");
    }

    #[test]
    fn close_curly() {
        let text = "}";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(CloseCurly));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "}");
    }

    #[test]
    fn open_bracket() {
        let text = "[";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(OpenBracket));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "[");
    }

    #[test]
    fn close_bracket() {
        let text = "]";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(CloseBracket));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "]");
    }

    #[test]
    fn left_angle() {
        let text = "<";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(LeftAngle));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "<");
    }

    #[test]
    fn double_left_angle() {
        let text = "<<";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleLeftAngle));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "<<");
    }

    #[test]
    fn left_angle_equals() {
        let text = "<=";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(LeftAngleEquals));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "<=");
    }

    #[test]
    fn right_angle() {
        let text = ">";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(RightAngle));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ">");
    }

    #[test]
    fn double_right_angle() {
        let text = ">>";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleRightAngle));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, ">>");
    }

    #[test]
    fn right_angle_equals() {
        let text = ">=";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(RightAngleEquals));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, ">=");
    }

    #[test]
    fn dot() {
        let text = ".";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Dot));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ".");
    }

    #[test]
    fn double_dot() {
        let text = "..";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleDot));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "..");
    }


    #[test]
    fn colon() {
        let text = ":";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Colon));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ":");
    }

    #[test]
    fn double_colon() {
        let text = "::";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleColon));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "::");
    }

    #[test]
    fn minus() {
        let text = "-";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Minus));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "-");
    }

    #[test]
    fn arrow() {
        let text = "->";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Arrow));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "->");
    }

    #[test]
    fn plus() {
        let text = "+";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Plus));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "+");
    }

    #[test]
    fn asterisk() {
        let text = "*";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Asterisk));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "*");
    }

    #[test]
    fn slash() {
        let text = "/";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Slash));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "/");
    }

    #[test]
    fn ampersand() {
        let text = "&";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Ampersand));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "&");
    }

    #[test]
    fn double_ampersand() {
        let text = "&&";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleAmpersand));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "&&");
    }

    #[test]
    fn pipe() {
        let text = "|";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Pipe));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "|");
    }

    #[test]
    fn double_pipe() {
        let text = "||";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoublePipe));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "||");
    }

    #[test]
    fn caret() {
        let text = "^";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Caret));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "^");
    }

    #[test]
    fn percent() {
        let text = "%";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Percent));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "%");
    }

    #[test]
    fn equals() {
        let text = "=";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Equals));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "=");
    }

    #[test]
    fn equals_equals() {
        let text = "==";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(DoubleEquals));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "==");
    }

    #[test]
    fn bang() {
        let text = "!";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Bang));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "!");
    }

    #[test]
    fn bang_equals() {
        let text = "!=";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(BangEquals));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 3, 2));
        assert_eq!(result.span.text, "!=");
    }
}