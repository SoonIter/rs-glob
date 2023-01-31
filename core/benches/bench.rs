use criterion::{criterion_group, criterion_main, Criterion};
use futures::executor::block_on;

fn glob_rs_glob(b: &mut Criterion) {
  b.bench_function("rs_glob", |b| {
    b.iter(|| block_on(rs_glob::rg("**/package.json")))
  });
}

fn glob_native_glob(b: &mut Criterion) {
  b.bench_function("native_glob", |b| {
    b.iter(|| {
      rs_glob::native_glob(
        &["**/package.json", "!**/node_modules", "!**/dist"],
        rs_glob::NativeGlobOptions {
          cwd: ".".to_string(),
        },
      )
    })
  });
}

criterion_group!(benches, glob_rs_glob, glob_native_glob);
criterion_main!(benches);
// #[tokio::main]
// async fn main() {
//   let res1 = rs_glob::rg("**/package.json").await;
//   let res2 = rs_glob::native_glob(
//     &["**/package.json", "!**/node_modules", "!**/dist"],
//     rs_glob::NativeGlobOptions {
//       cwd: ".".to_string(),
//     },
//   );
//   println!("{:?}\n{:?}\n{:?}\n{:?}", res1, res1.len(), res2, res2.len());
// }
