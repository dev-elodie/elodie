use std::cmp::PartialOrd;
use std::collections::HashMap;

use crate::ast::parse::Error::UnexpectedEndOfFile;
use crate::ast::parse::node::{Node, RootNode};
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{LiteralToken, OperatorToken, Token, TokenKind};
use crate::ast::token::TokenKind::Literal;

pub(crate) mod precedence;
pub(crate) mod node;
mod infix;
mod literal;
mod prefix;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfFile,
    UnexpectedToken {
        expected: TokenKind,
        got: Token,
    },
    UnsupportedNumber(String),
    UnsupportedToken(Token),
}

impl Error {
    pub(crate) fn eof() -> Self { Self::UnexpectedEndOfFile }
    pub(crate) fn unexpected(expected: TokenKind, got: Token) -> Self { Self::UnexpectedToken { expected, got } }
    pub(crate) fn unsupported(token: Token) -> Self { Self::UnsupportedToken(token) }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn parse(tokens: Vec<Token>) -> Result<RootNode> {
    Parser::new(tokens).parse()
}

struct Parser {
    tokens: Vec<Token>,
    precedence_map: HashMap<TokenKind, Precedence>,
}


impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut precedence_map = HashMap::new();

        precedence_map.insert(TokenKind::Operator(OperatorToken::Arrow), Precedence::Assignment);

        precedence_map.insert(TokenKind::Operator(OperatorToken::DoubleEqual), Precedence::Equality);
        precedence_map.insert(TokenKind::Operator(OperatorToken::BangEqual), Precedence::Equality);

        precedence_map.insert(TokenKind::Operator(OperatorToken::LeftAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::LeftAngleEqual), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::RightAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::RightAngleEqual), Precedence::Comparison);

        precedence_map.insert(TokenKind::Operator(OperatorToken::Plus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Minus), Precedence::Term);

        precedence_map.insert(TokenKind::Operator(OperatorToken::Asterisk), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Slash), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Percent), Precedence::Factor);

        precedence_map.insert(TokenKind::Operator(OperatorToken::OpenParen), Precedence::Call);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Dot), Precedence::Primary);
        precedence_map.insert(TokenKind::Operator(OperatorToken::DoubleColon), Precedence::Primary);

        let mut tokens = tokens;
        tokens.pop();
        tokens.reverse();

        Self {
            tokens,
            precedence_map,
        }
    }

    fn parse(&mut self) -> Result<RootNode> {
        let mut nodes = vec![];
        loop {
            if self.is_eof() { break; }
            nodes.push(self.parse_node(Precedence::None)?)
        }
        Ok(nodes.into())
    }

    pub(crate) fn parse_node(&mut self, precedence: Precedence) -> Result<Node> {
        let mut left = self.parse_prefix()?;

        while precedence < self.current_precedence()? {
            left = self.parse_infix(left)?;
        }
        Ok(left)
    }

    pub(crate) fn advance(&mut self) -> Result<Token> {
        self.tokens.pop().ok_or(Error::eof())
    }

    pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<Token> {
        self.current_expect(expected)?;
        self.advance()
    }

    pub(crate) fn consume_literal(&mut self, expected: LiteralToken) -> Result<Token> {
        self.current_expect_literal(expected)?;
        self.advance()
    }

    pub(crate) fn current(&self) -> Result<&Token> {
        self.tokens.last().ok_or(UnexpectedEndOfFile)
    }

    pub(crate) fn current_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.current()?;
        if got.kind == expected {
            Ok(())
        } else {
            return Err(Error::unexpected(expected, got.clone()));
        }
    }

    pub(crate) fn current_expect_literal(&self, literal: LiteralToken) -> Result<()> {
        self.current_expect(Literal(literal))
    }

    pub(crate) fn current_precedence(&self) -> Result<Precedence> {
        if self.is_eof() { return Ok(Precedence::None); };

        let current = self.current()?;
        let precedence = self.precedence_map.get(&current.kind).cloned();
        Ok(precedence.unwrap_or(Precedence::None))
    }

    pub(crate) fn peek(&self) -> Result<&Token> {
        if self.tokens.len() < 2 {
            return Err(Error::eof());
        }
        self.tokens.get(self.tokens.len() - 2).ok_or(Error::eof())
    }

    pub(crate) fn peek_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.peek()?;
        if got.kind == expected {
            Ok(())
        } else {
            return Err(Error::unexpected(expected, got.clone()));
        }
    }

    fn is_eof(&self) -> bool {
        self.tokens.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use LiteralToken::False;
    use OperatorToken::Plus;

    use crate::ast::lex::lex;
    use crate::ast::parse::{Error, Parser};
    use crate::ast::parse::precedence::Precedence;
    use crate::ast::parse::precedence::Precedence::Term;
    use crate::ast::token::{literal, LiteralToken, OperatorToken, separator};
    use crate::ast::token::LiteralToken::{Number, True};
    use crate::ast::token::SeparatorToken::Semicolon;

    #[test]
    fn advance_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.advance();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn advance() {
        let tokens = lex("1 + 2").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.advance().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "1");

        let result = parser.advance().unwrap();
        assert!(result.is_operator(Plus));

        let result = parser.advance().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "2");
    }

    #[test]
    fn consume_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.consume(literal(True));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn consume_but_unexpected_token() {
        let tokens = lex("false").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.consume(literal(True));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(True));
            assert!(got.is_literal(False));
        }
    }

    #[test]
    fn consume() {
        let tokens = lex("true 99").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.consume(literal(True)).unwrap();
        assert!(result.is_literal(True));

        let result = parser.consume(literal(Number)).unwrap();
        assert!(result.is_literal(Number));
    }


    #[test]
    fn current_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current() {
        let tokens = lex("true false").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current().unwrap();
        assert!(result.is_literal(True));
        assert_eq!(result.value(), "true");

        parser.advance().unwrap();
        let result = parser.current().unwrap();
        assert!(result.is_literal(False));
        assert_eq!(result.value(), "false");
    }

    #[test]
    fn current_expect_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current_expect() {
        let tokens = lex("true false").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.current_expect(literal(True));
        assert!(result.is_ok());

        parser.advance().unwrap();

        let result = parser.current_expect(literal(False));
        assert!(result.is_ok());
    }

    #[test]
    fn current_expect_but_different() {
        let tokens = lex("true").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.current_expect(literal(False));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(False));
            assert!(got.is_literal(True))
        }
    }

    #[test]
    fn current_precedence_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current_precedence();
        assert_eq!(result, Ok(Precedence::None))
    }

    #[test]
    fn current_precedence() {
        let tokens = lex("+").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current_precedence();
        assert_eq!(result, Ok(Term))
    }

    #[test]
    fn peek_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek_but_nothing_to_peek() {
        let tokens = lex("true").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek() {
        let tokens = lex("true false 1").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.peek().unwrap();
        assert!(result.is_literal(False));
        assert_eq!(result.value(), "false");

        parser.advance().unwrap();

        let result = parser.peek().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "1");
    }

    #[test]
    fn peek_expect_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.peek_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek_expect_but_nothing_to_peek() {
        let tokens = lex("true").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.peek_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile));
    }

    #[test]
    fn peek_expect() {
        let tokens = lex("true false 99").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.peek_expect(literal(False));
        assert!(result.is_ok());

        parser.advance().unwrap();

        let result = parser.peek_expect(literal(Number));
        assert!(result.is_ok());
    }

    #[test]
    fn peek_expect_but_different() {
        let tokens = lex("true 99").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.peek_expect(literal(False));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(False));
            assert!(got.is_literal(Number))
        }
    }
}