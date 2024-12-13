use crate::frontend::ast::node::{DefineTypeNode, Identifier};
use crate::frontend::ast::Generator;
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_define(
        &mut self,
        node: &parse::DefineDeclarationNode,
    ) -> ast::Result<ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        Ok(ast::Node::DefineType(DefineTypeNode {
            token: node.token.clone(),
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions: compiled_body
                .into_iter()
                .filter_map(|n| {
                    if let ast::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function)
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
}
