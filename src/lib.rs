#![deny(clippy::all)]

use uuid::Uuid;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn v7() -> String {
  Uuid::now_v7().to_string()
}
