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
#[napi]
pub fn rust_native_glob(pattern: Vec<String>) -> Vec<String> {
  native_glob(
    pattern
      .iter()
      .map(|x| x.as_str())
      .collect::<Vec<&str>>()
      .as_slice(),
    NativeGlobOptions {
      cwd: ".".to_string(),
    },
  )
}
