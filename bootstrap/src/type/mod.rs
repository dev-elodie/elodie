mod system;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeId(pub usize);

#[derive(Debug)]
pub struct TypeName(pub String);


pub struct Type {
    pub id: TypeId,
    pub name: TypeName,
    pub properties: HashMap<String, Property>,
}

pub struct Property {}


pub struct DefaultTypeIds {}

impl DefaultTypeIds {
    pub fn never() -> TypeId { TypeId(0) }
    pub fn any() -> TypeId { TypeId(1) }
    pub fn unit() -> TypeId { TypeId(2) }
    pub fn string() -> TypeId { TypeId(3) }
    pub fn number() -> TypeId { TypeId(4) }
    pub fn boolean() -> TypeId { TypeId(5) }
}