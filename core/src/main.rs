use glob_match::*;
use rs_glob::rg;

#[tokio::main]
async fn main() {
  // assert!(glob_match("**/*.txt", "y.txt"));
  assert!(glob_match("**/{y}.txt", "y.txt"));
  assert!(glob_match("**/{y}.txt", "y.txt"));
  assert!(glob_match("**/*.json", "package.json"));
  assert!(glob_match("**/{package}.json", "package.json"));
  assert!(glob_match("**/*.js", "b.js"));
  assert!(glob_match("**/*.json", "b.json"));
  // assert!(glob_match("**/package.json", "package.json"));
  // assert!(glob_match("!**/*.json", "package.json"));
}
