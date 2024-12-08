use crate::generate::c::emitter::Emitter;
use crate::generate::c::IfStatement;

impl Emitter {
    pub(crate) fn emit_if(&mut self, statement: &IfStatement) {
        self.emit_token("if");
        self.emit_token("(");
        self.emit_expression(&statement.condition);
        self.emit_token(")");
        self.emit_block_statement(&statement.then);
        if let Some(otherwise) = &statement.otherwise{
            self.emit_token("else");
            self.emit_block_statement(otherwise);
        }
    }
}