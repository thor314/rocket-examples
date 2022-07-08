//! Goal: A Pastebin-like site, per the Rocket tutorial https://rocket.rs/v0.5-rc/guide/pastebin/#pastebin-tutorial
//! Base functionality:
//! - get "/": return usage
//! - post "/": post a paste, get a paste_id
//! - delete "/<id>": delete a paste at id
//! - retreive "/<id>": get a paste at id

mod id;
mod index;
#[cfg(test)] mod tests;
use crate::{
  id::{delete, retreive},
  index::upload,
};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ { rocket::build().mount("/", routes![index::index, upload, delete, retreive]) }
