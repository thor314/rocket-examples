use rocket::{
  http::{ContentType, Status},
  local::blocking::Client,
  request::FromParam,
};

use super::{id::*, index::*, *};
// Rocket exposes some helpers we can use.

// set up client, and check the status and content
#[test]
fn test_index() {
  let client = Client::tracked(rocket()).unwrap();
  let response = client.get(uri!(index)).dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::Plain));
  assert_eq!(response.into_string(), Some(index().into()));
}

// set up the client, upload some data, query for it, delete it, query it's gone
#[test]
fn test_upload_retreive_delete() {
  let client = Client::tracked(rocket()).unwrap();
  let body = "hi mom";
  let id = upload_body(&client, body);
  // get id
  assert_eq!(Some(body.to_string()), retreive_upload(&client, &id));
  // delete body
  delete_upload(&client, &id);
  // check gone
  assert!(retreive_upload(&client, &id).is_none());
}

// convenience, upload body, check things ok, return the id
fn upload_body(client: &Client, body: &str) -> String {
  let resp = client.post(uri!(upload)).body(body).dispatch();
  assert_eq!(resp.status(), Status::Ok);
  assert_eq!(resp.content_type(), Some(ContentType::Plain));
  // extract the id from the long string
  let s = resp.into_string().unwrap();
  // println!("upload: {}", s);
  s.rfind('/').map(|i| &s[(i + 1)..]).map(|s| s.trim_end().to_string()).unwrap()
}

fn retreive_upload(client: &Client, id: &str) -> Option<String> {
  println!("id_smee: {}", id);
  let id = PasteId::from_param(id).expect("bogus id");
  let resp = client.get(uri!(retreive(id))).dispatch();
  if resp.status().class().is_success() {
    Some(resp.into_string().unwrap())
  } else {
    None
  }
}

fn delete_upload(client: &Client, id: &str) {
  let id = PasteId::from_param(id).unwrap();
  let resp = client.delete(uri!(delete(id))).dispatch();
  assert_eq!(resp.status(), Status::Ok);
}
