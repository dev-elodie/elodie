use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn consume_identifier(&self) -> crate::lexer::Result<Token> {
        let start = self.position();

        let mut text = self.consume_while(|c| {
            c.is_alphanumeric() || c == '_'
        })?;

        Ok(Token {
            kind: TokenKind::Identifier,
            span: TextSpan { start, end: self.position(), text },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::core::token::{Literal, Operator, TokenKind};
    use crate::lexer::Lexer;

    #[test]
    fn some_var() {
        let text = "some_var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, "some_var");
    }

    #[test]
    fn var() {
        let text = "var";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 4, 3));
        assert_eq!(result.span.text, "var");
    }

    #[test]
    fn console_log() {
        let text = "console.log('test')";
        let lexer = Lexer::new(text);

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 8, 7));
        assert_eq!(result.span.text, "console");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Operator::Dot));
        assert_eq!(result.span.start, (1, 8, 7));
        assert_eq!(result.span.end, (1, 9, 8));
        assert_eq!(result.span.text, ".");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Identifier);
        assert_eq!(result.span.start, (1, 9, 8));
        assert_eq!(result.span.end, (1, 12, 11));
        assert_eq!(result.span.text, "log");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Operator::OpenParen));
        assert_eq!(result.span.start, (1, 12, 11));
        assert_eq!(result.span.end, (1, 13, 12));
        assert_eq!(result.span.text, "(");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Literal(Literal::String));
        assert_eq!(result.span.start, (1, 13, 12));
        assert_eq!(result.span.end, (1, 19, 18));
        assert_eq!(result.span.text, "test");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Operator(Operator::CloseParen));
        assert_eq!(result.span.start, (1, 19, 18));
        assert_eq!(result.span.end, (1, 20, 19));
        assert_eq!(result.span.text, ")");

        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 20, 19));
        assert_eq!(result.span.end, (1, 20, 19));
        assert_eq!(result.span.text, "");
    }
}