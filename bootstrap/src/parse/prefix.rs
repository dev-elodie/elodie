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
    use crate::lex::lex;
    use crate::lex::token::{operator, test_token};
    use crate::lex::token::OperatorToken::{Bang, Minus, Plus};
    use crate::parse::node::{PrefixNode, PrefixOperator};
    use crate::parse::Node;
    use crate::parse::Parser;

    //
    //
    // macro_rules! parse_prefix {
    // ($($name:ident, $input:expr => $expected:expr,)*) => {
    //     $(
    //         #[test]
    //         fn $name() {
    //             println!("Test input: {:?}", $input);
    //             let tokens = lex($input).unwrap();
    //             let mut parser = Parser::new(&mut ctx,tokens);
    //             let result = parser.parse().unwrap();
    //             assert_eq!(result.len(), 1);
    //
    //             let Node::Prefix(PrefixNode{ ref operator, ref node }) = result[0] else { panic!() };
    //             assert_eq!(*operator, $ expected);
    //         }
    //     )*
    //     };
    // }
    //
    //
    // parse_prefix! {
    //     plus, "+2" => PrefixOperator::Plus(test_token(operator(Plus), "+")),
    //     negate, "-2" => PrefixOperator::Negate(test_token(operator(Minus), "-")),
    //     notl, "!true" => PrefixOperator::Not(test_token(operator(Bang), "!")),
    // }
    //
    //
    // macro_rules! parse_prefix_operator_test {
    // ($($name:ident, $input:expr => $expected:expr,)*) => {
    //     $(
    //         #[test]
    //         fn $name() {
    //             println!("Test input: {:?}", $input);
    //             let tokens = lex($input).unwrap();
    //             let mut parser = Parser::new(&mut ctx,tokens);
    //             let result = parser.parse_prefix_operator().unwrap();
    //             assert_eq!(result, $expected);
    //         }
    //     )*
    //     };
    // }
    //
    // parse_prefix_operator_test! {
    //     operator_plus, "+" => PrefixOperator::Plus(test_token(operator(Plus), "+")),
    //     operator_negate, "-" => PrefixOperator::Negate(test_token(operator(Minus), "-")),
    //     operator_not, "!" => PrefixOperator::Not(test_token(operator(Bang), "!")),
    // }
}