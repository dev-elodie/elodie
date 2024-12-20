use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::{new_ast, parse};
use crate::frontend::new_ast::{DeclareVariableNode, Generator, Identifier, Node, SPAN_NOT_IMPLEMENTED};
use crate::frontend::new_ast::node::AstNode;

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_variable(
        &mut self,
        node: &parse::VariableDeclarationNode,
    ) -> new_ast::Result<AstNode> {
        let variable = Identifier(node.identifier.0.clone());

        let node_type = if let Some(type_node) = node.r#type.as_ref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        let node = Rc::new(self.generate_node(node.node.deref())?);
        Ok(AstNode::new(Node::DeclareVariable(DeclareVariableNode {
            variable,
            value: node,
            value_type: node_type,
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
