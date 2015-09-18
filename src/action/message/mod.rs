use action::Action;
use config;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{
    Message,
    MessageBuilder
};

use handlebars::{
    Context,
    Handlebars,
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use state::State;
use self::error::Error;
use self::renderer_context::RendererContext;

mod error;
mod renderer_context;
#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = "context.uuid";
pub const CONTEXT_NAME: &'static str = "context.name";
pub const CONTEXT_LEN: &'static str = "context.len";
pub const MESSAGES: &'static str = "messages";
const MESSAGE_KEY: &'static str = "MESSAGE";

pub struct MessageAction {
    sender: Rc<RefCell<Box<ResponseSender<Response>>>>,
    uuid: String,
    name: Option<String>,
    values: Handlebars
}

impl MessageAction {
    pub fn new(sender: Rc<RefCell<Box<ResponseSender<Response>>>>, action: config::action::MessageAction) -> MessageAction {
        let config::action::MessageAction { uuid, name, message, values } = action;
        let mut handlebars = Handlebars::new();
        for (name, template) in values.into_iter() {
            handlebars.register_template(&name, template);
        }
        handlebars.register_template(MESSAGE_KEY, message);

        MessageAction {
            sender: sender,
            uuid: uuid,
            name: name,
            values: handlebars
        }
    }

    fn render_value(&self, key: &String, template_context: &Context) -> Result<String, Error> {
        let mut writer = Vec::new();
        {
            try!(self.values.renderw(key, &template_context, &mut writer));
        }
        let string = try!(String::from_utf8(writer));
        Ok(string)
    }

    fn render_values(&self, template_context: &Context) -> Result<BTreeMap<String, String>, Error> {
        let mut rendered_values = BTreeMap::new();
        for (key, _) in self.values.get_templates() {
            let rendered_value = try!(self.render_value(key, &template_context));
            rendered_values.insert(key.to_string(), rendered_value);
        }
        Ok(rendered_values)
    }

    fn render_message(&self, state: &State, context: &BaseContext) -> Result<Message, Error> {
        let template_context = {
            use handlebars::Context;
            let context = RendererContext::new(state, context);
            Context::wraps(&context)
        };

        let mut rendered_values = try!(self.render_values(&template_context));
        let message = rendered_values.remove(MESSAGE_KEY).expect(&format!("There is no '{}' key in the renderer key-value pairs", MESSAGE_KEY));
        let name = self.name.as_ref().map(|name| name.borrow());
        let message = MessageBuilder::new(&self.uuid, message)
                        .name(name)
                        .values(rendered_values)
                        .build();
        Ok(message)
    }
}

#[derive(Debug)]
pub struct MessageResponse {
    message: Message,
}

impl MessageResponse {
    pub fn message(&self) -> &Message {
        &self.message
    }
}

impl Action for MessageAction {
    fn execute(&self, _state: &State, _context: &BaseContext) {
        trace!("MessageAction: executed");
        match self.render_message(_state, _context) {
            Ok(message) => {
                let response = MessageResponse {
                    message: message,
                };
                self.sender.borrow_mut().send_response(Response::Message(response));
            },
            Err(error) => {
                error!("{}", error);
            }
        }
    }
}
