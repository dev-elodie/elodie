use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Object(Object),
    Function(Function),
    Unit,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Function(_) => "[Function]".to_string(),
            Value::Unit => "Unit".to_string()
        }
    }
}

#[derive(Clone)]
pub struct Function(pub Rc<dyn Fn(&[Value]) -> Value>);

impl PartialEq for Function{
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Function]")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    properties: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, key: &str, value: Value) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}
