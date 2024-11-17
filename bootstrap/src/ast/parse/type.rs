use crate::ast::parse::Error::UnknownType;
use crate::ast::parse::node::{TypeFunctionArgumentNode, TypeFunctionNode, TypeFundamentalNode, TypeNode};
use crate::ast::parse::Parser;
use crate::ast::token::OperatorToken::{Arrow, CloseParen, Colon, OpenParen};
use crate::ast::token::SeparatorToken::Comma;
use crate::ast::token::TokenKind::{Operator, Separator};

impl Parser {
    pub(crate) fn parse_type(&mut self) -> crate::ast::parse::Result<TypeNode> {
        let token = self.advance()?;
        match token.value() {
            "Bool" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Boolean(token))),
            "Number" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Number(token))),
            "String" => Ok(TypeNode::Fundamental(TypeFundamentalNode::String(token))),
            "fun" => Ok(TypeNode::Function(self.parse_function_type()?)),
            _ => Err(UnknownType(token))
        }
    }

    pub(crate) fn parse_function_type(&mut self) -> crate::ast::parse::Result<TypeFunctionNode> {
        self.consume_operator(OpenParen)?;

        let mut arguments = vec![];
        loop {
            if self.current()?.is_operator(CloseParen) {
                self.consume_operator(CloseParen)?;
                break;
            }
            arguments.push(self.parse_function_type_argument()?);
            self.consume_if(Separator(Comma))?;
        }

        let return_type = if !self.is_eof() && self.current()?.is_operator(Arrow) {
            self.consume(Operator(Arrow))?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(
            TypeFunctionNode {
                arguments,
                return_type,
            }
        )
    }

    pub(crate) fn parse_function_type_argument(&mut self) -> crate::ast::parse::Result<TypeFunctionArgumentNode> {
        let identifier = if self.peek()?.is_operator(Colon) {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.consume_if(Operator(Colon))?;

        let r#type = Box::new(self.parse_type()?);
        Ok(TypeFunctionArgumentNode { identifier, r#type })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::Error::UnknownType;
    use crate::ast::parse::node::{TypeFunctionArgumentNode, TypeFundamentalNode, TypeNode};
    use crate::ast::parse::Parser;

    #[test]
    fn not_a_type() {
        let tokens = lex("something_different").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type();
        let Err(UnknownType(_)) = result else { panic!() };
    }

    #[test]
    fn type_boolean() {
        let tokens = lex("Bool").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = result else { panic!() };
    }

    #[test]
    fn type_number() {
        let tokens = lex("Number").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result else { panic!() };
    }

    #[test]
    fn type_string() {
        let tokens = lex("String").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = result else { panic!() };
    }

    #[test]
    fn type_function_without_args_and_without_result() {
        let tokens = lex("fun()").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn type_function_without_args_and_with_result() {
        let tokens = lex("fun() -> Number").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments, vec![]);

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }

    #[test]
    fn type_function_single_named_arg_and_with_result() {
        let tokens = lex("fun(arg_1: Bool) -> Number").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else { panic!() };
        let Some(identifier) = identifier else { panic!() };
        assert_eq!(identifier.identifier(), "arg_1");

        let arg_type = r#type.as_ref();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = arg_type else { panic!() };

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }

    #[test]
    fn type_function_single_arg_and_with_result() {
        let tokens = lex("fun(Bool) -> Number").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else { panic!() };
        assert_eq!(*identifier, None);

        let arg_type = r#type.as_ref();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = arg_type else { panic!() };

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }
}