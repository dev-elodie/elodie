use std::rc::Rc;

use crate::common::SymbolId;
use crate::frontend::parse;
use crate::ir::infer::InferredType;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    DeclareVariable(DeclareVariableNode<'a>),
    Literal(LiteralNode<'a>),
}

impl<'a> Node<'a> {
    pub fn inferred_type(&self) -> InferredType {
        match self {
            Node::DeclareVariable(DeclareVariableNode { inferred_type, .. })
            | Node::Literal(LiteralNode::Boolean(LiteralBooleanNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::Number(LiteralNumberNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::String(LiteralStringNode { inferred_type, .. })) => inferred_type.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeclareVariableNode<'a> {
    pub parsed_node: &'a parse::DeclareVariableNode,
    pub symbol: SymbolId,
    pub node: Box<Node<'a>>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode<'a> {
    Boolean(LiteralBooleanNode<'a>),
    Number(LiteralNumberNode<'a>),
    String(LiteralStringNode<'a>),
}


#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode<'a> {
    pub parsed_node: &'a parse::LiteralBooleanNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode<'a> {
    pub parsed_node: &'a parse::LiteralNumberNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode<'a> {
    pub parsed_node: &'a parse::LiteralStringNode,
    pub inferred_type: InferredType,
}