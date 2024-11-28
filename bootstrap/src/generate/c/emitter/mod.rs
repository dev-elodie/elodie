use crate::generate::c;
use crate::generate::c::{Expression, Indent, Node};

mod directive;
mod function;
mod statement;

pub enum Error {}

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) fn emit(nodes: &[c::Node]) -> String {
    let mut emitter = Emitter {
        output: String::new(),
        indent: Indent::none(),
    };
    emitter.emit(nodes)
}

pub(crate) struct Emitter {
    pub(crate) output: String,
    pub(crate) indent: Indent,
}

impl Emitter {
    pub(crate) fn emit(mut self, nodes: &[c::Node]) -> String {
        for node in nodes {
            match node {
                Node::Directive(node) => self.emit_directive(node),
                Node::DeclareFunction(node) => self.emit_declare_function(node),
                Node::DeclareStruct(_) => unimplemented!(),
                Node::DefineFunction(node) => self.emit_define_function(node),
                Node::DefineStruct(_) => unimplemented!(),
                Node::DefineGlobalVariable(_) => unimplemented!(),
            }
        }
        self.output
    }

    pub(crate) fn emit_expression(&mut self, expression: &c::Expression) {
        match expression {
            Expression::CallFunction(expression) => self.emit_call_function(expression),
            Expression::Literal(_) => unimplemented!(),
            Expression::Unary(_) => unimplemented!(),
        }
    }

    pub(crate) fn emit_str(&mut self, str: &str) {
        self.output.push_str(str);
    }

    pub(crate) fn emit_token(&mut self, token: &str) {
        self.output.push_str(token);
        self.output.push_str(" ");
    }

    pub(crate) fn emit_line(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push_str("\n");
    }
}