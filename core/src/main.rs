use rs_glob::rg;

#[tokio::main]
async fn main() {
  let x = (rg("**/package.json")).await;
  for s in x {
    println!("{}", s);
  }
}
