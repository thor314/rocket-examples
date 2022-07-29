// ignore this
use std::{collections::HashMap, io};

use rocket::response::status::NotFound;

/// Sample Cacher implementation
struct Cacher {
  db:    HashMap<usize, Item>,
  cache: HashMap<usize, Item>,
}

impl Cacher {
  pub fn get_item(&mut self, id: usize) -> io::Result<Option<&Item>> {
    if self.cache.contains_key(&id) {
      return Ok(self.cache.get(&id));
    } else if self.db.contains_key(&id) {
      let item = self.db.get(&id).unwrap();
      self.cache.insert(id, item.clone());
      return Ok(Some(item));
    } else {
      Err(io::Error::new(io::ErrorKind::NotFound, "oof"))
    }
  }

  // update the item at key: id, with value. If no value exists, return Err, otherwise, return
  // Ok(previous_item).
  pub fn update_item(&mut self, id: usize, value: usize) -> io::Result<Item> {
    if self.cache.contains_key(&id) || self.db.contains_key(&id) {
      // replace value at both cache and db (eek, how efficient?)
      self.cache.insert(id, Item { value });
      let out = self.db.insert(id, Item { value }).unwrap();
      Ok(out)
    } else {
      Err(io::Error::new(io::ErrorKind::NotFound, "yikes"))
    }
  }
}

#[derive(Clone, Debug)]
struct Item {
  value: usize,
}

// further nonsense:
