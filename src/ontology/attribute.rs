use std::collections::HashMap;

#[derive(Debug)]
pub enum Attribute {
    List(HashMap<String, Attribute>),
    String(String),
    DateTime(String),
    Long(i64),
    Double(f64),
    Boolean(bool),
    ID(String),
}
