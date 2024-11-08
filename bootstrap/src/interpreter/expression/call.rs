use crate::ast::{CallArg, CallExpression, Expression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter{

    pub(crate) fn interpret_call(&mut self, call: &CallExpression) -> crate::interpreter::Result<Value> {
        let mut args: Vec<Value> = Vec::with_capacity(call.arguments.len());
        for arg in &call.arguments {
            args.push(self.interpret_call_arg(arg)?); // Now we can mutably borrow `self` without conflict
        }

        let function = if let Expression::PropertyAccess(ref access) = *call.expression {
            if let Some(boxed_expression) = &access.lhs {
                if let Expression::Identifier(object) = boxed_expression.as_ref() {
                    if let Some(Value::Object(object)) = self.scope.get(object.0.as_str()).as_ref() {
                        if let Expression::Identifier(function) = access.rhs.as_ref() {
                            if let Some(Value::Function(func)) = object.get_property(function.0.as_str()) {
                                Some(func)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(function) = function {
            return Ok(function.0(&args));
        }

        todo!()
    }

    fn interpret_call_arg(&mut self, arg: &CallArg) -> crate::interpreter::Result<Value> {
        self.interpret_expression(arg.value.as_ref())
    }

}