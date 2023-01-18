use crate::ontology::Attribute;
use crate::ontology::Event;
use crate::ontology::Extension;
use crate::ontology::Trace;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Log {
    pub version: String,
    pub features: Vec<String>,
    pub extensions: Vec<Extension>,
    pub attributes: HashMap<String, Attribute>,
    pub traces: Vec<Trace>,
    pub events: Vec<Event>,
}

impl Log {
    pub fn new(version: String, features: Vec<String>) -> Self {
        Self {
            version,
            features,
            extensions: Vec::new(),
            attributes: HashMap::new(),
            traces: Vec::new(),
            events: Vec::new(),
        }
    }
}
