use crate::ontology::Attribute;
use crate::ontology::Event;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Trace {
    pub attributes: HashMap<String, Attribute>,
    pub events: Vec<Event>,
}
