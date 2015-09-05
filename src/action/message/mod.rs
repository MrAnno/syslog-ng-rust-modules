use action::Action;
use config;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{Builder, Message};
use std::cell::RefCell;
use std::rc::Rc;
use state::State;

#[derive(Clone)]
pub struct MessageAction {
    pub sender: Rc<RefCell<Box<ResponseSender<Response>>>>,
    pub action: config::action::MessageAction
}

#[derive(Debug)]
pub struct MessageResponse {
    message: Message,
}

impl Action for MessageAction {
    fn execute(&self, _state: &State, _context: &BaseContext) {
        println!("MessageAction is executed");
        let mut message = Builder::new("d6621bd6-4898-4b8c-a4ff-36d0eed7d8dc")
                                .pair(".context.uuid", &_context.uuid().to_hyphenated_string())
                                .pair(".context.len", &_state.messages().len().to_string())
                                .build();
        if let Some(name) = _context.name() {
            message.insert(".context.name", name);
        }
        let response = MessageResponse {
            message: message,
        };
        self.sender.borrow_mut().send_response(Response::Message(response));
    }
}
