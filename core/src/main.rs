// use glob_match::*;
use rs_glob::*;

#[tokio::main]
async fn main() {
  println!("{:?}", rg("**/Cargo.toml").await);
  println!(
    "{:?}",
    native_glob(
      &["**/Cargo.toml", "!**/node_modules/**"],
      NativeGlobOptions {
        cwd: ".".to_string()
      }
    )
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn you_can_assert_eq() {
    assert!(glob_match("**/*.txt", "y.txt"));
    assert!(glob_match("**/{y}.txt", "y.txt"));
    assert!(glob_match("**/{y}.txt", "y.txt"));
    assert!(glob_match("**/*.json", "package.json"));
    assert!(glob_match("**/{package}.json", "package.json"));
    assert!(glob_match("**/*.js", "b.js"));
    assert!(glob_match("**/*.json", "b.json"));
  }
}
