use crate::common::{BaseType, DefaultTypeIds, TypeId};
use crate::common::Context;
use crate::frontend::{Ast, parse};
pub use crate::frontend::ast::node::*;
use crate::frontend::ast::scope::Scope;
use crate::frontend::parse::TypeNode;

mod r#let;
mod infix;
mod literal;
mod r#loop;
mod r#if;
mod identifier;
mod block;
mod function;
mod package;
mod from;
mod r#type;
mod define;
mod external;
mod string;
mod scope;
pub mod node;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub(crate) fn from(ctx: &mut Context, nodes: Vec<parse::Node>) -> Result<Ast> {
    let mut compiler = Compiler::new(ctx);
    compiler.compile(nodes)
}

pub(crate) struct Compiler<'a> {
    ctx: &'a mut Context,
    scope: Scope,
}

impl<'a> Compiler<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        let mut scope = Scope::new();

        Self {
            ctx,
            scope,
        }
    }
}

impl<'a> Compiler<'a> {
    pub(crate) fn compile(&mut self, nodes: Vec<parse::Node>) -> Result<Ast> {
        // 2 pass
        // populate symbol table
        // create ir

        let mut result = Vec::new();
        for node in &nodes {
            if !matches!(node, parse::Node::Nop) {
                result.push(self.compile_node(node)?);
            }
        }

        Ok(Ast { nodes: result })
    }

    pub(crate) fn compile_node(&mut self, node: &parse::Node) -> Result<node::Node> {
        match node {
            parse::Node::Block(block_node) => Ok(self.compile_block(block_node)?),
            parse::Node::Break(break_node) => Ok(self.compile_break(break_node)?),
            parse::Node::Continue(continue_node) => Ok(self.compile_continue(continue_node)?),
            parse::Node::DefineDeclaration(node) => Ok(self.compile_define(node)?),
            parse::Node::From(from_node) => Ok(self.compile_from(from_node)?),
            parse::Node::ExternalFunctionDeclaration(node) => self.compile_declare_external_function(node),
            parse::Node::FunctionDeclaration(declaration_node) => Ok(self.compile_declare_function(declaration_node)?),
            parse::Node::PackageDeclaration(declaration_node) => Ok(self.compile_declare_package(declaration_node)?),
            parse::Node::Identifier(identifier_node) => Ok(self.compile_identifier(identifier_node)?),
            parse::Node::VariableDeclaration(let_node) => Ok(self.compile_let(let_node)?),
            parse::Node::If(if_node) => Ok(self.compile_if(if_node)?),
            parse::Node::Infix(infix_node) => Ok(self.compile_infix(infix_node)?),
            parse::Node::StringInterpolation(node) => self.compile_interpolate_string(node),
            parse::Node::Itself(node) => Ok(self.compile_self(node)?),
            parse::Node::Literal(literal_node) => Ok(self.compile_literal(literal_node)?),
            parse::Node::Loop(loop_node) => Ok(self.compile_loop(loop_node)?),
            parse::Node::Return(return_node) => Ok(self.compile_function_return(return_node)?),
            parse::Node::TypeDeclaration(node) => Ok(self.compile_declare_type(node)?),
            _ => unimplemented!("{:?}", node)
        }
    }

    // FIXME temp hack until type node uses type ids from type table
    pub(crate) fn get_type_id(&mut self, type_node: &TypeNode) -> TypeId {
        match type_node {
            parse::TypeNode::Boolean(_) => self.ctx.type_table.get_base_type_id(&BaseType::Boolean),
            parse::TypeNode::Number(_) => self.ctx.type_table.get_base_type_id(&BaseType::Number),
            parse::TypeNode::String(_) => self.ctx.type_table.get_base_type_id(&BaseType::String),
            parse::TypeNode::Function(_) => DefaultTypeIds::never(),
            parse::TypeNode::Custom(_) => DefaultTypeIds::never()
        }
    }
}