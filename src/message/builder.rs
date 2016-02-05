use std::collections::BTreeMap;
use std::convert::Into;
use super::Message;

pub struct MessageBuilder {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>,
}

impl MessageBuilder {
    pub fn new<S: Into<String>>(uuid: &str, message: S) -> MessageBuilder {
        MessageBuilder {
            uuid: uuid.to_owned(),
            name: None,
            message: message.into(),
            values: BTreeMap::new(),
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: Option<S>) -> &mut MessageBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn values(&mut self, values: BTreeMap<String, String>) -> &mut MessageBuilder {
        self.values = values;
        self
    }

    pub fn pair(&mut self, key: &str, value: &str) -> &mut MessageBuilder {
        self.values.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            message: self.message.clone(),
            values: self.values.clone(),
        }
    }
}
