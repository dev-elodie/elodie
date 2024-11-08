use crate::core::span::TextSpan;
use crate::core::token::{Token, TokenKind};
use crate::core::token::Separator::{Comma, NewLine, Semicolon};
use crate::lexer::Lexer;

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


    pub(crate) fn consume_whitespace(&self) -> crate::lexer::Result<()> {
        self.consume_while(|c| self.is_whitespace(c))?;
        Ok(())
    }

    pub(crate) fn consume_separator(&self) -> crate::lexer::Result<Token> {
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
            _ => return Err(crate::lexer::Error::UnknownSeparator(text)),
        };

        Ok(Token {
            kind,
            span: TextSpan { start, end: self.position(), text },
        })
    }
}


#[cfg(test)]
mod test {
    use Separator::Comma;

    use crate::core::token::{Separator, TokenKind};
    use crate::core::token::Separator::{NewLine, Semicolon};
    use crate::lexer::Lexer;

    #[test]
    fn tab() {
        let text = "\t";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 2, 1));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, "")
    }

    #[test]
    fn whitespace() {
        let text = "     ";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::EOF);
        assert_eq!(result.span.start, (1, 6, 5));
        assert_eq!(result.span.end, (1, 6, 5));
        assert_eq!(result.span.text, "")
    }

    #[test]
    fn comma() {
        let text = ",";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Comma));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ",");
    }

    #[test]
    fn semicolon() {
        let text = ";";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(Semicolon));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (1, 2, 1));
        assert_eq!(result.span.text, ";");
    }

    #[test]
    fn new_line() {
        let text = "\n\n\n";
        let lexer = Lexer::new(text);
        let result = lexer.advance().unwrap();
        assert_eq!(result.kind, TokenKind::Separator(NewLine));
        assert_eq!(result.span.start, (1, 1, 0));
        assert_eq!(result.span.end, (3, 1, 3));
        assert_eq!(result.span.text, "\n\n\n");
    }
}