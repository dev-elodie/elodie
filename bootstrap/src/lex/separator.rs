use crate::lex::Lexer;
use crate::lex::token::{TextSpan, Token, TokenKind};
use crate::lex::token::SeparatorToken::{Comma, NewLine, Semicolon};

impl Lexer<'_> {
    pub(crate) fn is_whitespace(&self, c: char) -> bool {
        match c {
            | '\u{0009}' // \t
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space
            => true,
            _ => false
        }
    }

    pub(crate) fn is_separator(&self, c: char) -> bool {
        matches!(c,  ',' | ';' | '\n')
    }


    pub(crate) fn consume_whitespace(&self) -> crate::lex::Result<()> {
        self.consume_while(|c| self.is_whitespace(c))?;
        Ok(())
    }

    pub(crate) fn consume_separator(&mut self) -> crate::lex::Result<Token> {
        let start = self.position();
        let mut text = String::from(self.consume_next()?);

        let kind = match text.as_str() {
            "," => TokenKind::Separator(Comma),
            ";" => TokenKind::Separator(Semicolon),
            "\n" => {
                let additional = self.consume_while(|c| c == '\n')?;
                text.push_str(&additional);
                self.current_line.borrow_mut().0 += additional.len();
                self.current_column.borrow_mut().0 = 1;
                TokenKind::Separator(NewLine)
            }
            _ => return Err(crate::lex::Error::UnknownSeparator(text)),
        };

        Ok(Token {
            kind,
            span: TextSpan { start, end: self.position(), value: self.ctx.string_table.insert(text.as_str()) },
        })
    }
}


#[cfg(test)]
mod test {
    use crate::common::Context;
    use crate::lex::Lexer;
    use crate::lex::token::SeparatorToken::{Comma, NewLine, Semicolon};
    use crate::lex::token::TokenKind;

    #[test]
    fn tab() {
        let text = "\t";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 2, 1));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(ctx.get_str(result.value()), "")
    }

    #[test]
    fn whitespace() {
        let text = "     ";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 6, 5));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(ctx.get_str(result.value()), "")
    }

    #[test]
    fn comma() {
        let text = ",";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Comma));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(ctx.get_str(result.value()), ",");
    }

    #[test]
    fn semicolon() {
        let text = ";";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Semicolon));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(ctx.get_str(result.value()), ";");
    }

    #[test]
    fn new_line() {
        let text = "\n\n\n";
        let mut ctx = Context::new();
        let mut lexer = Lexer::new(&mut ctx, text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(NewLine));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (3, 1, 3));
        assert_eq!(ctx.get_str(result.value()), "\n\n\n");
    }
}