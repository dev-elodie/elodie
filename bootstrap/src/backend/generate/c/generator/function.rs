use crate::backend::generate::c;
use crate::backend::generate::c::{CallFunctionStatement, CallFunctionStatementResult, DeclareFunctionNode, DeclareVariableStatement, Expression, Indent, LiteralExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::backend::generate::c::generator::Generator;
use crate::backend::generate::c::Statement::CallFunction;
use crate::frontend::ast;
use crate::frontend::ast::node::LoadValueNode;

impl Generator {
    pub(crate) fn generate_declare_function(&mut self, node: &ast::DeclareFunctionNode) -> c::generator::Result<DeclareFunctionNode> {
        unimplemented!("{node:#?}")
    }

    pub(crate) fn generate_call_function(&mut self, node: &ast::CallFunctionNode) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_table.get(node.function.0).to_string();

        let mut result = vec![];

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);


        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments: arguments.into(),
                result: Some(CallFunctionStatementResult {
                    indent: Indent::none(),
                    identifier: "arg_2".to_string(),
                    r#type: "double".to_string(),
                }),
            })
        );

        Ok(result)
    }

    pub(crate) fn generate_call_function_with_result(&mut self, node: &ast::CallFunctionNode, call_result: CallFunctionStatementResult) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_table.get(node.function.0).to_string();

        let mut result = vec![];

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);

        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments: arguments.into(),
                result: Some(call_result),
            })
        );

        Ok(result)
    }


    pub(crate) fn generate_call_function_of_package(&mut self, node: &ast::CallFunctionOfPackageNode) -> c::generator::Result<Vec<Statement>> {
        let mut result = vec![];

        let std = self.string_table.get(node.package.segments[0]).to_string();
        let io = self.string_table.get(node.package.segments[1]).to_string();
        let function = self.string_table.get(node.function.0).to_string();

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);

        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: format!("{std}_{io}_{function}"),
                arguments: arguments.into(),
                result: None,
            })
        );

        return Ok(result);
    }

    fn generate_call_arguments(&mut self, args: &[ast::Node]) -> c::generator::Result<(Vec<Statement>, Vec<Expression>)> {
        let mut statements = vec![];
        let mut arguments = vec![];

        for arg in args {
            let arg_identifier = self.scope.push_argument();

            if let ast::Node::LoadValue(LoadValueNode { identifier }) = arg {
                // if self.type_table.is_string(ty) {
                //     statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                //         indent: Indent::none(),
                //         identifier: arg_identifier.to_string(),
                //         r#type: "const char *".to_string(),
                //         expression: Expression::Variable(VariableExpression { indent: Indent::none(), identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table) }),
                //     }));
                //
                //
                //     arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                //     continue;
                // }
            }

            if let ast::Node::Literal(ast::LiteralNode::String(str)) = arg {
                statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: arg_identifier.to_string(),
                    r#type: "const char *".to_string(),
                    expression: Expression::Literal(LiteralExpression::String(LiteralStringExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(str.value()).to_string(),
                    })),
                }));

                arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                continue;
            }

            // to_string + concatenation
            if let ast::Node::InterpolateString(node) = arg {
                let (s, a) = self.interpolate_string(node)?;
                statements.extend(s);
                arguments.push(a);
                continue;
            }

            if let ast::Node::CallFunction(node) = arg {
                let s = self.generate_call_function(node)?;
                statements.extend(s);
                arguments.push(Expression::Variable(VariableExpression { indent: Indent::none(), identifier: "arg_2".to_string() }));
                continue;
            }

            unimplemented!("{arg:#?}")
        }

        Ok((statements, arguments))
    }
}