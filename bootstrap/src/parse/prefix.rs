use SeparatorToken::NewLine;

use crate::common::{is_pascal_snake_case, is_snake_case};
use crate::lex::token::{KeywordToken, OperatorToken, SeparatorToken};
use crate::lex::token::LiteralToken::{False, Number, String, True};
use crate::lex::token::TokenKind::{Keyword, Operator};
use crate::parse::{Error, Parser};
use crate::parse::Error::UnsupportedToken;
use crate::parse::node::{Node, PrefixNode, PrefixOperator};
use crate::parse::Node::{PackageDeclaration, TypeDeclaration};
use crate::parse::node::Node::{Break, Continue, FunctionDeclaration, If, Let, Loop, Return};
use crate::parse::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_prefix(&mut self) -> crate::parse::Result<Node> {
        loop {
            if self.is_eof() {
                return Ok(Node::Nop);
            }

            let is_new_line = self.current()?.is_separator(NewLine);
            if !is_new_line {
                break;
            }
            let _ = self.advance()?;
        }

        let current = self.current()?;
        match &current.kind {
            Operator(operator) => {
                match operator {
                    OperatorToken::Plus | OperatorToken::Minus | OperatorToken::Bang => {
                        let operator = self.parse_prefix_operator()?;
                        Ok(Node::Prefix(PrefixNode {
                            operator,
                            node: Box::new(self.parse_node(Precedence::None)?),
                        }))
                    }
                    OperatorToken::OpenCurly => Ok(Node::Block(self.parse_block()?)),
                    OperatorToken::OpenParen => Ok(Node::Tuple(self.parse_tuple()?)),
                    _ => Err(Error::unsupported(self.advance()?))
                }
            }
            Keyword(keyword) => {
                match keyword {
                    KeywordToken::Break => Ok(Break(self.parse_break()?)),
                    KeywordToken::Continue => Ok(Continue(self.parse_continue()?)),
                    KeywordToken::Export => Ok(self.parse_export()?),
                    KeywordToken::From => Ok(Node::From(self.parse_from()?)),
                    KeywordToken::Function => Ok(FunctionDeclaration(self.parse_function_declaration()?)),
                    KeywordToken::If => Ok(If(self.parse_if()?)),
                    KeywordToken::Let => Ok(Let(self.parse_let()?)),
                    KeywordToken::Loop => Ok(Loop(self.parse_loop()?)),
                    KeywordToken::Package => Ok(PackageDeclaration(self.parse_package_declaration()?)),
                    KeywordToken::Return => Ok(Return(self.parse_return()?)),
                    KeywordToken::Type => Ok(TypeDeclaration(self.parse_type_declaration()?)),
                    _ => Err(Error::unsupported(self.advance()?))
                }
            }
            _ => match current {
                _ if current.is_literal(Number) => Ok(Node::Literal(self.parse_literal_number()?)),
                _ if current.is_literal(True) => Ok(Node::Literal(self.parse_literal_true()?)),
                _ if current.is_literal(False) => Ok(Node::Literal(self.parse_literal_false()?)),
                _ if current.is_literal(String) => Ok(Node::Literal(self.parse_literal_string()?)),
                _ if current.is_identifier() => {
                    if is_snake_case(self.ctx.get_str(current.value())) {
                        Ok(Node::Identifier(self.parse_identifier()?))
                    } else if is_pascal_snake_case(self.ctx.get_str(current.value())) {
                        Ok(Node::Type(self.parse_type()?))
                    } else {
                        unreachable!()
                    }
                }
                _ => Err(Error::unsupported(self.advance()?))
            }
        }
    }

    pub(crate) fn parse_prefix_operator(&mut self) -> crate::parse::Result<PrefixOperator> {
        let token = self.advance()?;
        match &token.kind {
            Operator(operator) => match operator {
                OperatorToken::Plus => Ok(PrefixOperator::Plus(token)),
                OperatorToken::Minus => Ok(PrefixOperator::Negate(token)),
                OperatorToken::Bang => Ok(PrefixOperator::Not(token)),
                _ => Err(UnsupportedToken(token))
            }
            _ => Err(UnsupportedToken(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::{LiteralNode, Node, parse};
    use crate::parse::node::{PrefixNode, PrefixOperator};
    use crate::parse::Node::Literal;

    #[test]
    fn plus() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "+2").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Node::Prefix(PrefixNode { ref operator, ref node }) = result[0] else { panic!() };
        assert!(matches!(*operator, PrefixOperator::Plus(_)));

        let Literal(LiteralNode::Number(node)) = node.deref() else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "2");
    }

    #[test]
    fn negate() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "-2").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Node::Prefix(PrefixNode { ref operator, ref node }) = result[0] else { panic!() };
        assert!(matches!(*operator, PrefixOperator::Negate(_)));

        let Literal(LiteralNode::Number(node)) = node.deref() else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "2");
    }

    #[test]
    fn not() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "!false").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Node::Prefix(PrefixNode { ref operator, ref node }) = result[0] else { panic!() };
        assert!(matches!(*operator, PrefixOperator::Not(_)));

        let Literal(LiteralNode::Boolean(node)) = node.deref() else { panic!() };
        assert_eq!(node.value(), false);
    }
}