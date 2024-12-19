use std::rc::Rc;

use crate::common::StringTable;
use crate::frontend::{Ast, ast};
use crate::ir::analyse::node::Node;
use crate::ir::context::Context;
use crate::ir::symbol::{SymbolId, SymbolName, SymbolTable};

mod literal;
mod declare;
mod r#type;

pub(crate) struct Inference<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
}

impl<'a> Inference<'a> {
    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
        }
    }

    pub(crate) fn infer(&mut self, ast: Ast) -> crate::ir::analyse::Result<Vec<Node>> {
        let mut nodes = vec![];
        for node in ast.nodes {
            nodes.push(self.infer_node(Rc::new(node))?);
        }
        Ok(nodes)
    }

    fn infer_node(&mut self, node: Rc<ast::Node>) -> crate::ir::analyse::Result<Node> {
        match node {
            // ast::Node::DeclareVariable(node) => self.infer_declare_variable(node),
            // ast::Node::Literal(node) => self.infer_literal(node),
            _ => unimplemented!("{node:#?}")
        }
    }

    fn register_argument(&mut self, name: SymbolName) -> SymbolId {
        // self.ctx.symbol_table.register_argument(name)
        todo!()
    }

    // fn register_function(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_function(name)
    // }
    //
    // fn register_package(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_package(name)
    // }
    //
    // fn register_type(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_type(name)
    // }
    //
    fn register_variable(&mut self, name: SymbolName) -> SymbolId {
        self.symbol_table.register_variable(name)
    }
}
