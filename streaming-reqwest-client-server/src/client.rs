// use req

use std::future;

use futures::StreamExt;

use crate::message::Message;

mod message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut res = reqwest::get("http://127.0.0.1:8000/events")
    .await?
    .bytes_stream()
    .map(|x| x.unwrap())
    .filter(|x| future::ready(&**x != b":\n" && &**x != b"\n"));
  while let Some(item) = res.next().await {
    println!("Chunk: {item:?}");
    let item: Message = item.into();
    println!("Parsed item: {item:?}");
  }
  Ok(())
}
