use std::collections::BTreeMap;

use context::ContextMap;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandle, Request};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};
use dispatcher::response::ResponseSender;

pub struct RequestReactor {
    handlers: BTreeMap<RequestHandle, Box<EventHandler<Request, ContextMap>>>,
    demultiplexer: Demultiplexer<Request>,
    context_map: ContextMap,
    responder: Box<ResponseSender>
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<Request>, context_map: ContextMap, responder: Box<ResponseSender>) -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            context_map: context_map,
            handlers: BTreeMap::new(),
            responder: responder
        }
    }
}

impl Reactor<ContextMap> for RequestReactor {
    type Event = Request;
    fn handle_events(&mut self) {
        while let Some(request) = self.demultiplexer.select() {
            trace!("RequestReactor: got event");
            if let Some(handler) = self.handlers.get_mut(&request.handle()) {
                handler.handle_event(request, &mut self.context_map);
            } else {
                trace!("RequestReactor: no handler found for event");
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, ContextMap>>) {
        self.handlers.insert(handler.handle(), handler);
    }
    fn remove_handler_by_handle(&mut self, handler: &RequestHandle) {
        self.handlers.remove(handler);
    }
}
