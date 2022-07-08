use std::{
  borrow::Cow,
  path::{Path, PathBuf},
};

use rand::Rng;
use rocket::{
  request::FromParam,
  response::content::RawText,
  tokio::fs::{remove_file, File},
};
// use rocket::data::{Data, ToByteUnit};
// use rocket::http::uri::Absolute;
// use rocket::response::content::RawText;
// use rocket::tokio::fs::{self, File};
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Retrieve some data at id
#[get("/<id>")]
pub async fn retreive(id: PasteId<'_>) -> Option<RawText<File>> {
  File::open(id.file_path()).await.map(RawText).ok()
}

/// delete some data at id
#[delete("/<id>")]
pub async fn delete(id: PasteId<'_>) -> Option<()> { remove_file(id.file_path()).await.ok() }

/// A unique-ish id for each paste. Derive UriDisplayPath for moo.
/// Implement `FromParam` for PasteId, so we can use it as a function parameter.
#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> FromParam<'a> for PasteId<'a> {
  type Error = &'a str;

  fn from_param(param: &'a str) -> Result<Self, Self::Error> {
    param.chars().all(|c| c.is_ascii_alphanumeric()).then(|| PasteId(param.into())).ok_or(param)
  }
}
impl<'a> PasteId<'a> {
  pub(crate) fn new(id_length: usize) -> Self {
    let mut rng = rand::thread_rng();
    let id: String =
      std::iter::repeat_with(|| BASE62[rng.gen_range(0..62)] as char).take(id_length).collect();
    Self(Cow::Owned(id))
  }

  pub(crate) fn file_path(&self) -> PathBuf {
    let root = "/home/thor/r/play/rocket_pastebin_example/store";
    Path::new(root).join(self.0.as_ref())
  }
}
