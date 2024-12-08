use std::ops::Deref;

use crate::generate::c;
use crate::generate::c::{DeclareVariableStatement, Indent, Statement, VariableExpression};
use crate::generate::c::generator::Generator;
use crate::generate::c::generator::scope::Variable;
use crate::ir::{DeclareVariableNode, LiteralNode, LoadValueNode};
use crate::ir::Node::Literal;

impl Generator {
    pub(crate) fn generate_declare_variable(&mut self, node: &DeclareVariableNode) -> crate::generate::c::generator::Result<Vec<Statement>> {

        let variable = self.scope.push_variable(&node.identifier);

        if let Literal(LiteralNode::String(string)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "const char *".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::String(c::LiteralStringExpression {
                            indent: Indent::none(),
                            value: self.string_table.get(string.value).to_string(),
                        })
                    ),
                })
            ]);
        }


        if let Literal(LiteralNode::Number(number)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "double".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::Double(c::LiteralDoubleExpression {
                            indent: Indent::none(),
                            value: self.string_table.get(number.value).parse::<f64>().unwrap(),
                        })
                    ),
                })
            ]);
        }

        if let Literal(LiteralNode::Bool(boolean)) = &node.value.deref() {
            return Ok(vec![
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: variable.to_string(&self.string_table),
                    r#type: "_Bool".to_string(),
                    expression: c::Expression::Literal(
                        c::LiteralExpression::Bool(c::LiteralBooleanExpression {
                            indent: Indent::none(),
                            value: boolean.value,
                        })
                    ),
                })
            ]);
        }

        unimplemented!("{node:#?}");
    }

    pub(crate) fn generate_load_value(&mut self, node: &LoadValueNode) -> c::generator::Result<c::Expression> {
        Ok(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: self.scope.get_variable(&node.identifier).unwrap().to_string(&self.string_table) }))
    }
}