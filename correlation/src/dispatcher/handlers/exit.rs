// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::response::ResponseSender;
use dispatcher::Response;
use dispatcher::request::{Request, RequestHandle};
use reactor::{EventHandler, SharedData};
use Event;
use Template;

#[derive(Default)]
pub struct ExitEventHandler;

impl<'a, E, T> EventHandler<Request<E>, SharedData<'a, E, T>> for ExitEventHandler where E: Event, T: Template<Event=E> {
    fn handle_event(&mut self, event: Request<E>, data: &mut SharedData<E, T>) {
        if let Request::Exit = event {
            data.responder.send_response(Response::Exit);
        } else {
            unreachable!("An ExitEventHandler should only receive Exit events");
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Exit
    }
}
