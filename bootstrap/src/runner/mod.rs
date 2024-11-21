use std::collections::HashMap;
use std::ops::Deref;

use crate::ast::{CalculationOperator, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CompareOperator, Node, SourceFile};
use crate::runner::scope::Scope;
use crate::runner::value::Value;

mod scope;
mod value;
mod declaration;
mod r#loop;
mod r#if;
mod block;
mod call;

#[derive(Debug)]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Runner {
    scope: Scope,
    pub interrupt: Option<Interrupt>,
}


#[derive(Debug, Clone)]
pub enum Interrupt {
    Break(Value),
    Continue,
    Return(Value),
}

impl Runner {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            interrupt: None,
        }
    }

    pub fn run(&mut self, source_file: SourceFile) -> Result<Value> {
        for node in &source_file.body {
            self.run_node(node)?;
        }
        Ok(Value::Unit)
    }

    pub(crate) fn run_node(&mut self, node: &Node) -> Result<Value> {
        match node {
            Node::BreakLoop(break_node) => self.run_break(break_node),

            Node::DeclareVariable(declaration) => self.run_variable_declaration(declaration),
            Node::DeclareFunction(declaration) => self.run_function_declaration(declaration),
            Node::DeclarePackage(declaration) => {
                let value = self.run_package_declaration(declaration)?;
                let Value::Package(package) = value else { panic!() };
                self.scope.insert(package.identifier.clone(), Value::Package(package));
                Ok(Value::Unit)
            }

            Node::CallFunctionOfObject(CallFunctionOfObjectNode { object, function, arguments }) => {
                // let some_arg_value = if let Node::CallFunction(arg_1) = &arguments[0] {
                //     let value = self.run_call_function(arg_1)?.clone();
                //     Some(value)
                // } else {
                //     None
                // };
                //
                let mut args = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    args.push(self.run_node(arg)?);
                }

                let Value::Object(object) = self.scope.get(object.deref()).unwrap() else { panic!() };
                let func = object.get_property_host_function(function).unwrap();

                if let Node::UseIdentifier(load_varialbe_node) = &arguments[0] {
                    let value = self.scope.get(load_varialbe_node.identifier.0.as_str()).unwrap().clone();
                    return func.0(&[value]);
                }

                if let Node::ValueString(arg_1) = &arguments[0] {
                    return func.0(&[Value::String(arg_1.to_string())]);
                }

                return func.0(args.as_slice());
            }

            Node::CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) => {
                let mut args = HashMap::with_capacity(arguments.len());

                let root = package.first().unwrap();
                let Value::Package(root_package) = self.scope.get(root).unwrap().clone() else { panic!() };

                //FIXME recursively get package
                let func = if package.len() == 1 {
                    root_package.get_function(function).unwrap()
                } else {
                    let target_package = root_package.packages.get(package.last().unwrap().0.as_str()).unwrap();
                    target_package.get_function(function).unwrap()
                };

                let mut counter = 0;
                for arg in arguments {
                    // args.push(self.run_node(arg)?);
                    let arg_node = func.arguments.get(counter).unwrap();

                    let name = arg_node.identifier.0.clone();
                    // FIXME resolve  name from definition
                    args.insert(name, self.run_node(arg)?);
                    counter += 1;
                }


                if arguments.len() > 0 {
                    if let Node::UseIdentifier(load_varialbe_node) = &arguments[0] {
                        let value = self.scope.get(load_varialbe_node.identifier.0.as_str()).unwrap().clone();
                        let mut args = HashMap::new();
                        args.insert("message".to_string(), Value::String("you are on he right track".to_string()));
                        return self.run_node_call(func.clone(), args);
                    }
                }

                self.run_node_call(func.clone(), args)
            }

            Node::CallFunction(function_node) => self.run_node_call_function(function_node),
            Node::ReturnFromFunction(node) => {
                let value = self.run_node(node.node.deref())?;
                self.interrupt(Interrupt::Return(value.clone()));
                Ok(value)
            }

            Node::ValueString(value) => Ok(Value::String(value.to_string())),
            Node::ValueNumber(value) => Ok(Value::Number(value.clone())),
            Node::ValueBoolean(value) => Ok(Value::Bool(value.clone())),
            Node::Loop(loop_node) => self.run_loop(loop_node),
            Node::If(if_node) => self.run_if(if_node),

            Node::Block(block_node) => self.run_block(block_node),

            Node::Compare(compare_node) => {
                let left = self.run_node(compare_node.left.deref())?;
                let right = self.run_node(compare_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match compare_node.operator {
                        CompareOperator::GreaterThan => Ok(Value::Bool(l > r)),
                        CompareOperator::Equal => Ok(Value::Bool(l == r)),
                        CompareOperator::NotEqual => Ok(Value::Bool(l != r))
                    };
                }

                if let (Value::Bool(l), Value::Bool(r)) = (&left, &right) {
                    return match compare_node.operator {
                        CompareOperator::GreaterThan => Ok(Value::Bool(l > r)),
                        CompareOperator::Equal => Ok(Value::Bool(l == r)),
                        CompareOperator::NotEqual => Ok(Value::Bool(l != r))
                    };
                }

                unimplemented!()
            }

            Node::Calculate(calculation_node) => {
                let left = self.run_node(calculation_node.left.deref())?;
                let right = self.run_node(calculation_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match calculation_node.operator {
                        CalculationOperator::Multiply => Ok(Value::Number(l * r)),
                        CalculationOperator::Add => Ok(Value::Number(l + r))
                    };
                }

                if let (Value::String(l), Value::String(r)) = (&left, &right) {
                    return match calculation_node.operator {
                        CalculationOperator::Add => Ok(Value::String(l.clone() + r)),
                        _ => todo!()
                    };
                }

                unimplemented!()
            }
            Node::UseIdentifier(load_variable) => {
                let value = self.scope.get(load_variable.identifier.0.as_str()).unwrap().clone();
                Ok(value)
            }
            _ => unimplemented!("{:?}", node)
        }
    }

    pub fn interrupt(&mut self, loop_interrupt: Interrupt) {
        self.interrupt = Some(loop_interrupt)
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt = None
    }
}