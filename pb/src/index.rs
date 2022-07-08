use rocket::{
  data::{Data, ToByteUnit},
  http::uri::Absolute,
};

use crate::id::PasteId;

const ID_LENGTH: usize = 4;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

/// Get usage string
#[get("/")]
pub fn index() -> &'static str {
  r"
    USAGE
      POST /
          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content
          EXAMPLE: curl --data-binary @file.txt http://localhost:8000
      GET /<id>
          retrieves the content for the paste with id `<id>`
    "
}

/// upload some data, return an id
#[post("/", data = "<data>")]
pub async fn upload(data: Data<'_>) -> std::io::Result<String> {
  let id = PasteId::new(ID_LENGTH);
  // assert!(false);
  data.open(128usize.kibibytes()).into_file(id.file_path()).await?;
  let uri = uri!(HOST, crate::id::retreive(id)).to_string();
  Ok(uri)
}
