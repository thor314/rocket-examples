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
use rocket::{
  form::Form,
  fs::{relative, FileServer},
  response::stream::{Event, EventStream},
  serde::{Deserialize, Serialize},
  tokio::{
    select,
    sync::broadcast::{channel, error::RecvError, Sender},
  },
  Shutdown, State, FromForm,
};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
  #[field(validate = len(..30))]
  pub room:     String,
  #[field(validate = len(..20))]
  pub username: String,
  pub message:  String,
}
