// use req

use std::{future, time::Duration};

use futures::StreamExt;
use reqwest::Client;
use rocket::http::hyper::body::Bytes;
use tokio::time::sleep;

use crate::message::Message;

mod message;

/// Method for the client to send a message to the server
async fn send_message(client: &Client) -> anyhow::Result<()> {
  client
    .post("http://127.0.0.1:8000/message")
    .form(&[("room", "23"), ("message", "Hi Bob"), ("username", "Al")])
    .send()
    .await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut res = reqwest::get("http://127.0.0.1:8000/events")
    .await?
    .bytes_stream()
    .map(|x| x.unwrap())
    .filter(|x:&Bytes| future::ready(&**x != b":\n" && &**x != b"\n"));

  // spawn a task to send messages every 5 seconds
  tokio::spawn(async move {
    let client = Client::new();
    loop {
      send_message(&client).await.unwrap();
      sleep(Duration::from_secs(5)).await;
    }
  });

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
