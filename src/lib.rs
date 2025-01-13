#![deny(clippy::all)]

use rand::prelude::*;
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

#[napi]
#[inline(always)]
pub fn v4() -> String {
  Uuid::new_v4().to_string()
}

const NANO_ALPHABET: [char; 32] = [
  '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
  'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-',
];
const NANO_MASK: usize = 31;
const NANO_SIZE: usize = 21;
const NANO_STEP: usize = 8 * NANO_SIZE / 5;

#[napi]
#[inline(always)]
pub fn nano() -> String {
  let mut id = String::with_capacity(NANO_SIZE);
  let mut rng = rand_chacha::ChaCha20Rng::from_entropy();

  loop {
    let mut bytes: Vec<u8> = vec![0; NANO_STEP];
    rng
      .try_fill_bytes(&mut bytes)
      .expect("Failed to fill bytes");

    for &byte in &bytes {
      let byte = byte as usize & NANO_MASK;
      id.push(NANO_ALPHABET[byte]);

      if id.len() == NANO_SIZE {
        return id;
      }
    }
  }
}
