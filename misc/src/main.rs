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

mod utils;
// #[tokio::main]
// async
fn main() -> Result<()> { Ok(()) }
