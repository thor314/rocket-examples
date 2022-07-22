//! A batteries-included binary template.

// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::Result;
use clap::Parser;
use futures::{
  channel::mpsc,
  executor::{self, ThreadPool},
  StreamExt,
};
use message::Message;
use utils::MyError;
use validator::{Validate, ValidationError};

mod message;
mod utils;
// mod client;
#[macro_use] extern crate rocket;

use rocket::{
  form::Form,
  fs::{relative, FileServer},
  response::stream::{Event, EventStream},
  serde::{Deserialize, Serialize},
  tokio::{
    select,
    sync::broadcast::{channel, error::RecvError, Sender},
  },
  Shutdown, State,
};

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
// curl http://127.0.0.1:8000/events
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
  let mut rx = queue.subscribe();
  EventStream! {
      loop {
          let msg = select! {
              msg = rx.recv() => match msg {
                  Ok(msg) => msg,
                  Err(RecvError::Closed) => break,
                  Err(RecvError::Lagged(_)) => continue,
              },
              _ = &mut end => break,
          };

          yield Event::json(&msg);
      }
  }
}

/// Receive a message from a form submission and broadcast it to any receivers.
// curl -d "room=23&username=Al&message=Hi Bob" http://127.0.0.1:8000/message
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
  // A send 'fails' if there are no active subscribers. That's okay.
  let _res = queue.send(form.into_inner());
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .manage(channel::<Message>(1024).0)
    .mount("/", routes![post, events])
    .mount("/", FileServer::from(relative!("static")))
}