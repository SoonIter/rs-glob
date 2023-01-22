use rs_glob::rg;
fn main() {
  let x = rg("**/*.json");
  for s in x {
    println!("{}",s);
  }
}
