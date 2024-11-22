use crate::lex::token::OperatorToken;
use crate::lex::token::TokenKind::Operator;
use crate::parse::Error::UnsupportedToken;
use crate::parse::node::{InfixNode, InfixOperator, Node};
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_infix(&mut self, left: Node) -> crate::parse::Result<InfixNode> {
        let operator = self.parse_infix_operator()?;

        let precedence = self.current_precedence()?;

        let right = if let InfixOperator::Call(token) = &operator {
            Node::Tuple(self.parse_tuple_call(token.clone())?)
        } else if let InfixOperator::Arrow(_) = &operator {
            Node::Block(self.parse_block_inner()?)
        } else {
            self.parse_node(precedence)?
        };

        Ok(InfixNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub(crate) fn parse_infix_operator(&mut self) -> crate::parse::Result<InfixOperator> {
        let token = self.advance()?;
        match &token.kind {
            Operator(operator) => match operator {
                OperatorToken::OpenParen => Ok(InfixOperator::Call(token)),
                OperatorToken::Plus => Ok(InfixOperator::Add(token)),
                OperatorToken::Minus => Ok(InfixOperator::Subtract(token)),
                OperatorToken::Asterisk => Ok(InfixOperator::Multiply(token)),
                OperatorToken::Slash => Ok(InfixOperator::Divide(token)),
                OperatorToken::Percent => Ok(InfixOperator::Modulo(token)),
                OperatorToken::Equal => Ok(InfixOperator::Assign(token)),
                OperatorToken::DoubleEqual => Ok(InfixOperator::Equal(token)),
                OperatorToken::BangEqual => Ok(InfixOperator::NotEqual(token)),
                OperatorToken::LeftAngle => Ok(InfixOperator::LessThan(token)),
                OperatorToken::LeftAngleEqual => Ok(InfixOperator::LessThanOrEqual(token)),
                OperatorToken::RightAngle => Ok(InfixOperator::GreaterThan(token)),
                OperatorToken::RightAngleEqual => Ok(InfixOperator::GreaterThanOrEqual(token)),
                OperatorToken::Colon => Ok(InfixOperator::TypeAscription(token)),
                OperatorToken::Arrow => Ok(InfixOperator::Arrow(token)),
                OperatorToken::Dot => Ok(InfixOperator::AccessProperty(token)),
                OperatorToken::DoubleColon => Ok(InfixOperator::AccessPackage(token)),
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
    use crate::lex::token::OperatorToken::*;
    use crate::parse::{parse, TypeFundamentalNode, TypeNode};
    use crate::parse::node::{InfixNode, InfixOperator, LiteralNode, TupleNode};
    use crate::parse::Node::{Infix, Type};
    use crate::parse::node::Node::{Identifier, Literal};

    #[test]
    fn identifier_with_type() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "u: Bool").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Infix(InfixNode { left, operator, right }) = &result[0] else { panic!() };
        let InfixOperator::TypeAscription(_) = operator else { panic!() };

        let Identifier(identifier) = left.as_ref() else { panic!() };
        assert_eq!(ctx.get_str(identifier.value()), "u");

        let Type(type_node) = right.as_ref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!() };
    }

//     macro_rules! parse_infix {
//     ($($name:ident, $input:expr => $expected:expr,)*) => {
//         $(
//             static ctx: Context = Context::default();
//
//             #[test]
//             fn $name() {
//                 println!("Test input: {:?}", $input);
//                 let tokens = lex($input).unwrap();
//                 let mut parser = Parser::new(&mut ctx,tokens);
//                 let result = parser.parse().unwrap();
//                 assert_eq!(result.len(), 1);
//
//                 let Infix(InfixNode{ ref left, ref operator, ref right }) = result[0] else { panic!() };
//
//                 let Literal(LiteralNode::Number(node)) = left.deref() else {panic!()};
//                 assert_eq!(node.value().unwrap(), 1.0);
//
//                 assert_eq!(*operator, $ expected);
//
//                 let Literal(LiteralNode::Number(node)) = right.deref() else {panic!()};
//                 assert_eq!(node.value().unwrap(), 2.0);
//             }
//         )*
//     };
// }
//
//     parse_infix! {
//         add, "1 + 2" => InfixOperator::Add(test_token_with_offset(operator(Plus), "+", 2)),
//         subtract, "1 - 2" => InfixOperator::Subtract(test_token_with_offset(operator(Minus), "-", 2)),
//         multiply, "1 * 2" => InfixOperator::Multiply(test_token_with_offset(operator(Asterisk), "*", 2)),
//         divide, "1 / 2" => InfixOperator::Divide(test_token_with_offset(operator(Slash), "/", 2)),
//         modulo, "1 % 2" => InfixOperator::Modulo(test_token_with_offset(operator(Percent), "%", 2)),
//         greater_than, "1 > 2" => InfixOperator::GreaterThan(test_token_with_offset(operator(RightAngle), ">", 2)),
//         greater_than_or_equal, "1 >= 2" => InfixOperator::GreaterThanOrEqual(test_token_with_offset(operator(RightAngleEqual), ">=", 2)),
//         less_than, "1 < 2" => InfixOperator::LessThan(test_token_with_offset(operator(LeftAngle), "<", 2)),
//         less_than_or_equal, "1 <= 2" => InfixOperator::LessThanOrEqual(test_token_with_offset(operator(LeftAngleEqual), "<=", 2)),
//         equal, "1 == 2" => InfixOperator::Equal(test_token_with_offset(operator(DoubleEqual), "==", 2)),
//         not_equal, "1 != 2" => InfixOperator::NotEqual(test_token_with_offset(operator(BangEqual), "!=", 2)),
//     }
//
//     macro_rules! parse_infix_operator_test {
//     ($($name:ident, $input:expr => $expected:expr,)*) => {
//         $(
//             #[test]
//             fn $name() {
//                 println!("Test input: {:?}", $input);
//                 let tokens = lex($input).unwrap();
//                 let mut parser = Parser::new(&mut ctx,tokens);
//                 let result = parser.parse_infix_operator().unwrap();
//                 assert_eq!(result, $expected);
//             }
//         )*
//     };
// }
//
//     parse_infix_operator_test! {
//         operator_add, "+" => InfixOperator::Add(test_token(operator(Plus), "+")),
//         operator_subtract, "-" => InfixOperator::Subtract(test_token(operator(Minus), "-")),
//         operator_multiply, "*" => InfixOperator::Multiply(test_token(operator(Asterisk), "*")),
//         operator_divide, "/" => InfixOperator::Divide(test_token(operator(Slash), "/")),
//         operator_modulo, "%" => InfixOperator::Modulo(test_token(operator(Percent), "%")),
//         operator_equal, "==" => InfixOperator::Equal(test_token(operator(DoubleEqual), "==")),
//         operator_not_equal, "!=" => InfixOperator::NotEqual(test_token(operator(BangEqual), "!=")),
//         operator_less_than, "<" => InfixOperator::LessThan(test_token(operator(LeftAngle), "<")),
//         operator_less_than_or_equal, "<=" => InfixOperator::LessThanOrEqual(test_token(operator(LeftAngleEqual), "<=")),
//         operator_greater_than, ">" => InfixOperator::GreaterThan(test_token(operator(RightAngle), ">")),
//         operator_greater_than_or_equal, ">=" => InfixOperator::GreaterThanOrEqual(test_token(operator(RightAngleEqual), ">=")),
//     }

    #[test]
    fn call_function_of_object() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "console.log()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let InfixNode { left, operator, right } = result[0].as_infix();
        let Identifier(node) = left.deref() else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "console");

        let InfixOperator::AccessProperty(_) = operator else { panic!() };

        let InfixNode { left, operator, right } = right.as_infix();
        let Identifier(node) = left.deref() else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "log");

        let InfixOperator::Call(_) = operator else { panic!() };

        let TupleNode { nodes, .. } = right.as_tuple();
        assert_eq!(*nodes, vec![]);
    }


    #[test]
    fn call_without_arguments() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "test()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let InfixNode { left, operator, right } = &result[0].as_infix();
        let identifier = left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "test");

        let InfixOperator::Call(_) = operator else { panic!() };

        let TupleNode { nodes, .. } = right.as_tuple();
        assert_eq!(*nodes, vec![]);
    }

    #[test]
    fn call_with_argument() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "test('elodie')").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let InfixNode { left, operator, right } = &result[0].as_infix();
        let identifier = left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "test");

        let InfixOperator::Call(_) = operator else { panic!() };

        let TupleNode { nodes, .. } = right.as_tuple();
        assert_eq!(nodes.len(), 1);

        let Some(Literal(LiteralNode::String(arg_1))) = &nodes.first() else { panic!() };
        assert_eq!(ctx.get_str(arg_1.value()), "elodie");
    }

    #[test]
    fn call_package_function() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "some_package::some_function()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let InfixNode { left, operator, right } = &result[0].as_infix();
        let identifier = left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "some_package");

        let InfixOperator::AccessPackage(_) = operator else { panic!() };

        let InfixNode { left, operator, right } = right.as_infix();
        let identifier = left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "some_function");

        let InfixOperator::Call(_) = operator else { panic!() };

        let TupleNode { nodes, .. } = right.as_tuple();
        assert_eq!(*nodes, vec![]);
    }

    #[test]
    fn instantiate_type_without_properties() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "Point()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_infix();
    }
}