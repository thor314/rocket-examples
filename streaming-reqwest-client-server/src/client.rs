// use req

use std::future;

use futures::StreamExt;
use tokio::{join, task::JoinHandle};

use crate::message::Message;

mod message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut res = reqwest::get("http://127.0.0.1:8000/events")
    .await?
    .bytes_stream()
    .map(|x| x.unwrap())
    .filter(|x| future::ready(&**x != b":\n" && &**x != b"\n"));

  // let h1: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
  tokio::spawn(async move {
    while let Some(item) = res.next().await {
      println!("h1 Chunk: {item:?}");

      let s = std::str::from_utf8(&*item).unwrap().trim().split_once(":").unwrap().1;
      println!("h1 string item: {s:?}");
      let msg: Message = serde_json::from_str(&s).unwrap();
      println!("h1 Parsed item: {msg:?}");
    }
    println!("h1 closing");
    // Ok(())
  })
  .await?;

  // experiment sending string ends annoyingly: quotes get triple backslashed.

  // let mut res_str = reqwest::get("http://127.0.0.1:8000/events_str")
  //   .await?
  //   .bytes_stream()
  //   .map(|x| x.unwrap())
  //   .filter(|x| future::ready(&**x != b":\n" && &**x != b"\n"));

  // let h2: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
  //   while let Some(item) = res_str.next().await {
  //     println!("h2 Chunk: {item:?}");

  //     let s = std::str::from_utf8(&*item)?;
  //     println!("h2 string item: {s:?}");
  //     let msg: Message = serde_json::from_str(&s)?;
  //     println!("h2 Parsed item: {msg:?}");
  //   }
  //   println!("h2 closing");
  //   Ok(())
  // });

  // let _ = join!(h1, h2);
  Ok(())
}
