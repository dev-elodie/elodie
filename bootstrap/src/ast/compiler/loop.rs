use crate::ast;
use crate::ast::{BreakNode, ContinueNode, LoopNode, Node, parse};
use crate::ast::compiler::Compiler;
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_break(&mut self, node: &parse::BreakNode) -> crate::ast::compiler::Result<ast::Node> {
        if node.result.is_none() {
            Ok(Node::Break(BreakNode { body: None, return_type: DefaultTypeIds::unit() }))
        } else {
            let body = Some(Box::new(self.compile_node(node.result.as_ref().unwrap())?));
            Ok(Node::Break(BreakNode {
                body,
                return_type: DefaultTypeIds::never(),
            }))
        }
    }

    pub(crate) fn compile_continue(&mut self, _node: &parse::ContinueNode) -> crate::ast::compiler::Result<ast::Node> {
        Ok(Node::Continue(ContinueNode {}))
    }

    pub(crate) fn compile_loop(&mut self, node: &parse::LoopNode) -> ast::compiler::Result<ast::Node> {
        let mut body = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            body.push(self.compile_node(node)?)
        }

        Ok(
            Node::Loop(LoopNode {
                body,
                return_type: DefaultTypeIds::unit(),
            })
        )
    }
}