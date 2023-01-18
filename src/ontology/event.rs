use crate::ontology::Attribute;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Event {
    pub attributes: HashMap<String, Attribute>,
}
