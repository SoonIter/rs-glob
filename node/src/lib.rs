#![deny(clippy::all)]
use rs_glob::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn glob(s1: String, s2: String) -> bool {
  glob_match(s1.as_str(), s2.as_str())
}
