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
use utils::MyError;
use validator::{Validate, ValidationError};

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
mod utils;

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

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct BroadcastMessage {
  pub from:     String,
  #[field(validate = len(..30))]
  pub room:     String,
  #[field(validate = len(..20))]
  pub username: String,
  pub message:  String,
}
/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
/// See https://docs.rs/rocket/0.5.0-rc.2/rocket/response/stream/macro.stream.html
/// Take: Any series of statements
/// Expand into an `impl Stream<Item = T>`.
#[get("/events")]
async fn events(queue: &State<Sender<BroadcastMessage>>, mut end: Shutdown) -> EventStream![] {
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
      yield Event::json(&msg); // Stream<Item = Event> // with data = BroadcastMessage
    }
  }
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<BroadcastMessage>, queue: &State<Sender<BroadcastMessage>>) {
  // A send 'fails' if there are no active subscribers. That's okay.
  let _res = queue.send(form.into_inner());
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .manage(channel::<BroadcastMessage>(1024).0)
    .mount("/", routes![post, events])
    .mount("/", FileServer::from(relative!("static")))
}
