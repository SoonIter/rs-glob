use rs_glob::*;

fn main() {
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
  fn correctness_of_glob_match() {
    assert!(glob_match("**/*.txt", "y.txt"));
    assert!(glob_match("**/{y}.txt", "y.txt"));
    assert!(glob_match("**/y.txt", "y.txt"));
    assert!(glob_match("**/*.json", "package.json"));
    assert!(glob_match("**/{package}.json", "package.json"));
    assert!(glob_match("**/package.json", "package.json"));
    assert!(glob_match("**/*.js", "b.js"));
    assert!(glob_match("**/*.json", "b.json"));
  }
}
