use crate::generate::c;
use crate::generate::c::{CallFunctionStatement, CallFunctionStatementResult, DeclareArrayStatement, DeclareVariableStatement, Expression, Indent, LiteralExpression, LiteralIntExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::generate::c::Expression::{Literal, Variable};
use crate::generate::c::generator::Generator;
use crate::generate::c::Statement::CallFunction;
use crate::ir::{CallFunctionNode, CallFunctionOfPackageNode, InterpolateStringNode, LiteralNode, LoadValueNode, Node};

impl Generator {
    pub(crate) fn generate_call_function(&mut self, node: &CallFunctionNode) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_table.get(node.function.0).to_string();

        let mut result = vec![];

        let arguments = self.generate_call_arguments(&node.arguments)?.into_boxed_slice();
        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments,
                result: None,
            })
        );

        Ok(result)
    }

    pub(crate) fn generate_call_function_of_package(&mut self, node: &CallFunctionOfPackageNode) -> c::generator::Result<Vec<Statement>> {
        let mut result = vec![];

        let std = self.string_table.get(node.package.segments[0]).to_string();
        let io = self.string_table.get(node.package.segments[1]).to_string();
        let function = self.string_table.get(node.function.0).to_string();

        let mut arguments = vec![];

        for arg in &node.arguments {
            let arg_identifier = self.scope.push_argument();

            if let Node::LoadValue(LoadValueNode { identifier, ty }) = arg {
                if self.type_table.is_string(ty) {
                    result.push(Statement::DeclareVariable(DeclareVariableStatement {
                        indent: Indent::none(),
                        identifier: arg_identifier.to_string(),
                        r#type: "const char *".to_string(),
                        expression: Expression::Variable(VariableExpression { indent: Indent::none(), identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table) }),
                    }));


                    arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }))
                }
            }

            if let Node::Literal(LiteralNode::String(str)) = arg {
                result.push(Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: arg_identifier.to_string(),
                    r#type: "const char *".to_string(),
                    expression: Expression::Literal(LiteralExpression::String(LiteralStringExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(str.value).to_string(),
                    })),
                }));

                arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }))
            }

            // to_string + concatenation
            if let Node::InterpolateString(InterpolateStringNode { nodes }) = arg {
                for node in nodes {
                    if let Node::LoadValue(LoadValueNode { identifier, ty }) = node {
                        if self.type_table.is_number(ty) {
                            result.push(Statement::DeclareArray(DeclareArrayStatement {
                                indent: Indent::none(),
                                identifier: arg_identifier.to_string(),
                                r#type: "char".to_string(),
                                size: 20,
                            }));

                            result.push(Statement::CallFunction(
                                CallFunctionStatement {
                                    indent: Indent::none(),
                                    identifier: format!("snprintf"),
                                    arguments: Box::new([
                                        Variable(VariableExpression {
                                            indent: Indent::none(),
                                            identifier: arg_identifier.to_string(),
                                        }),
                                        Literal(LiteralExpression::Int(LiteralIntExpression {
                                            indent: Indent::none(),
                                            value: 20,
                                        })),
                                        Literal(LiteralExpression::String(LiteralStringExpression {
                                            indent: Indent::none(),
                                            value: "%.0f".to_string(),
                                        })),
                                        Variable(VariableExpression {
                                            indent: Indent::none(),
                                            identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table),
                                        }),
                                    ]),
                                    result: None,
                                })
                            );

                            arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }))
                        }

                        if self.type_table.is_boolean(ty) {
                            result.push(Statement::CallFunction(
                                CallFunctionStatement {
                                    indent: Indent::none(),
                                    identifier: "core_bool_to_string".to_string(),
                                    arguments: Box::new([
                                        Variable(VariableExpression {
                                            indent: Indent::none(),
                                            identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table),
                                        }),
                                    ]),
                                    result: Some(CallFunctionStatementResult {
                                        indent: Indent::none(),
                                        identifier: arg_identifier.to_string(),
                                        r#type: "const char *".to_string(),
                                    }),
                                })
                            );

                            arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                        }
                    }
                }
            }
        }


        // let arguments = self.generate_call_arguments(&node.arguments)?.into_boxed_slice();

        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: format!("{std}_{io}_{function}"),
                arguments: arguments.into(),
                result: None,
            })
        );

        return Ok(result);

        unimplemented!()
    }

    fn generate_call_arguments(&mut self, nodes: &[Node]) -> c::generator::Result<Vec<Expression>> {
        let mut result = vec![];
        for node in nodes {
            result.push(self.generate_expression(node)?)
        }
        Ok(result)
    }
}