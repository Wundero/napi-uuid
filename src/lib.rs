#![deny(clippy::all)]

use uuid::Uuid;

#[macro_use]
extern crate napi_derive;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
#[inline(always)]
pub fn v7() -> String {
  Uuid::now_v7().to_string()
}
