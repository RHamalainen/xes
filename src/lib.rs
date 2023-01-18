//! Read and write eXtensible Event Stream (XES) format.

pub(crate) mod ontology;

pub use ontology::Attribute;
pub use ontology::Event;
pub use ontology::Extension;
pub use ontology::Log;
pub use ontology::Trace;
use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::Event as XmlEvent;
use std::collections::HashMap;
use std::path::Path;

const ATTRIBUTE_TAGS: [&str; 7] = [
    "list", "string", "datetime", "long", "double", "boolean", "id",
];

fn parse_attribute(attributee: &roxmltree::Node) -> (String, Attribute) {
    let key = attributee.attribute("key").unwrap().to_owned();
    match attributee.attribute("value") {
        Some(value) => {
            let value = value.to_owned();
            let value = match attributee.tag_name().name() {
                "string" => Attribute::String(value),
                "datetime" => Attribute::DateTime(value),
                "long" => Attribute::Long(value.parse().unwrap()),
                "double" => Attribute::Double(value.parse().unwrap()),
                "boolean" => Attribute::Boolean(value.parse().unwrap()),
                "id" => Attribute::ID(value),
                _ => panic!(),
            };
            (key, value)
        }
        None => {
            let mut sub_attributes = HashMap::new();
            for sub_attributee in attributee
                .children()
                .filter(|e| ATTRIBUTE_TAGS.contains(&e.tag_name().name()))
            {
                let (k, v) = parse_attribute(&sub_attributee);
                sub_attributes.insert(k, v);
            }
            let value = Attribute::List(sub_attributes);
            (key, value)
        }
    }
}

fn parse_event(evente: &roxmltree::Node) -> Event {
    let mut attributes = HashMap::new();
    for attributee in evente
        .children()
        .filter(|e| ATTRIBUTE_TAGS.contains(&e.tag_name().name()))
    {
        let (key, value) = parse_attribute(&attributee);
        attributes.insert(key, value);
    }
    Event { attributes }
}

fn parse_trace(tracee: &roxmltree::Node) -> Trace {
    let mut attributes = HashMap::new();
    for attributee in tracee
        .children()
        .filter(|e| ATTRIBUTE_TAGS.contains(&e.tag_name().name()))
    {
        let (key, value) = parse_attribute(&attributee);
        attributes.insert(key, value);
    }
    let mut events = Vec::new();
    for evente in tracee.children().filter(|e| e.tag_name().name() == "event") {
        events.push(parse_event(&evente));
    }
    Trace { attributes, events }
}

fn parse_log(loge: &roxmltree::Node) -> Log {
    let version = loge.attribute("version").unwrap().parse().unwrap();
    let features = loge
        .attribute("features")
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .map(String::from)
        .collect();
    let mut log = Log::new(version, features);
    for exte in loge
        .children()
        .filter(|e| e.tag_name().name() == "extension")
    {
        let name = exte.attribute("name").unwrap().to_owned();
        let prefix = exte.attribute("prefix").unwrap().to_owned();
        let uri = exte.attribute("uri").unwrap().to_owned();
        log.extensions.push(Extension { name, prefix, uri });
    }
    for attributee in loge
        .children()
        .filter(|e| ATTRIBUTE_TAGS.contains(&e.tag_name().name()))
    {
        let (key, value) = parse_attribute(&attributee);
        log.attributes.insert(key, value);
    }
    for tracee in loge.children().filter(|e| e.tag_name().name() == "trace") {
        log.traces.push(parse_trace(&tracee));
    }
    for evente in loge.children().filter(|e| e.tag_name().name() == "event") {
        log.events.push(parse_event(&evente));
    }
    log
}

/// Transform `XES`-file to Rust representation.
pub fn read<P: AsRef<Path>>(path: P) -> Vec<Log> {
    let text = std::fs::read_to_string(path).unwrap();
    let document = roxmltree::Document::parse(&text).unwrap();
    let mut logs = Vec::new();
    for log in document
        .root()
        .children()
        .filter(|e| e.tag_name().name() == "log")
    {
        logs.push(parse_log(&log));
    }
    logs
}

fn write_extension(extension: &Extension, events: &mut Vec<XmlEvent>) {
    let mut exte = BytesStart::new("extension");
    exte.push_attribute(("name", extension.name.as_str()));
    exte.push_attribute(("prefix", extension.prefix.as_str()));
    exte.push_attribute(("uri", extension.uri.as_str()));
    events.push(XmlEvent::Start(exte));
    events.push(XmlEvent::End(BytesEnd::new("extension")));
}

fn write_attribute(attribute: (&String, &Attribute), events: &mut Vec<XmlEvent>) {
    let (k, v) = attribute;
    let element_name = match v {
        Attribute::List(_) => "list",
        Attribute::String(_) => "string",
        Attribute::Long(_) => "long",
        Attribute::Double(_) => "double",
        Attribute::DateTime(_) => "date",
        Attribute::Boolean(_) => "boolean",
        Attribute::ID(_) => "id",
    };
    let mut attribute = BytesStart::new(element_name);
    attribute.push_attribute(("key", k.as_str()));
    match v {
        Attribute::List(list) => {
            events.push(XmlEvent::Start(BytesStart::new("list")));
            for sub_attribute in list {
                write_attribute(sub_attribute, events);
            }
            events.push(XmlEvent::End(BytesEnd::new("list")));
        }
        Attribute::String(value) => {
            attribute.push_attribute(("value", value.as_str()));
        }
        Attribute::Long(value) => {
            attribute.push_attribute(("value", value.to_string().as_str()));
        }
        Attribute::Double(value) => {
            attribute.push_attribute(("value", value.to_string().as_str()));
        }
        Attribute::DateTime(value) => {
            attribute.push_attribute(("value", value.to_string().as_str()));
        }
        Attribute::Boolean(value) => {
            attribute.push_attribute(("value", value.to_string().as_str()));
        }
        Attribute::ID(value) => {
            attribute.push_attribute(("value", value.to_string().as_str()));
        }
    }
    events.push(XmlEvent::Start(attribute));
    events.push(XmlEvent::End(BytesEnd::new(element_name)));
}

fn write_event(event: &Event, events: &mut Vec<XmlEvent>) {
    events.push(XmlEvent::Start(BytesStart::new("event")));
    for attribute in &event.attributes {
        write_attribute(attribute, events);
    }
    events.push(XmlEvent::End(BytesEnd::new("event")));
}

fn write_trace(trace: &Trace, events: &mut Vec<XmlEvent>) {
    events.push(XmlEvent::Start(BytesStart::new("trace")));
    for attribute in &trace.attributes {
        write_attribute(attribute, events);
    }
    for event in &trace.events {
        write_event(event, events);
    }
    events.push(XmlEvent::End(BytesEnd::new("trace")));
}

fn write_log(log: &Log, events: &mut Vec<XmlEvent>) {
    let mut loge = BytesStart::new("log");
    loge.push_attribute(("version", log.version.as_str()));
    loge.push_attribute(("features", log.features.join(",").as_str()));
    events.push(XmlEvent::Start(loge));
    for extension in &log.extensions {
        write_extension(extension, events);
    }
    for attribute in &log.attributes {
        write_attribute(attribute, events);
    }
    for trace in &log.traces {
        write_trace(trace, events);
    }
    for event in &log.events {
        write_event(event, events);
    }
    events.push(XmlEvent::End(BytesEnd::new("log")));
}

/// Transform Rust representation to `XES`-file.
pub fn write<P: AsRef<Path>>(log: &Log, path: P) {
    use quick_xml::Writer;
    use std::io::Cursor;
    let mut events = Vec::new();
    write_log(log, &mut events);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    for event in events {
        writer.write_event(event).unwrap();
    }
    let contents = writer.into_inner().into_inner();
    std::fs::write(path, contents).unwrap();
}
