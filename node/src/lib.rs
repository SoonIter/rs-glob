#![deny(clippy::all)]
use rs_glob::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn glob(pattern: String, st: String) -> bool {
  glob_match(&pattern, &st)
}

#[napi]
pub async fn rs_glob(pattern: String) -> Vec<String> {
  rg(&pattern).await
}
