use criterion::{criterion_group, criterion_main, Criterion};
// use futures::executor::block_on;
use wax::Glob;

fn wax_glob() -> Vec<String> {
  let glob = Glob::new("**/package.json").unwrap();
  glob
    .walk(".")
    .not(["**/node_modules/**", "**/dist/**"])
    .unwrap()
    .filter_map(|item| {
      if let Ok(i) = item {
        Some(i.path().to_string_lossy().to_string())
      } else {
        None
      }
    })
    .collect()
}

fn rs_native_glob() -> Vec<String> {
  rs_glob::native_glob(
    &["**/package.json", "!**/node_modules", "!**/dist"],
    rs_glob::NativeGlobOptions {
      cwd: ".".to_string(),
    },
  )
}

// async fn git_ls_tree_glob() -> Vec<String> {
//   rs_glob::rg("**/package.json").await
// }

fn bench_wax_glob(b: &mut Criterion) {
  b.bench_function("wax_glob", |b| b.iter(wax_glob));
}

// fn glob_rs_glob(b: &mut Criterion) {
//   b.bench_function("rs_git_ls_glob", |b| {
//     b.iter(|| block_on(git_ls_tree_glob()))
//   });
// }

fn glob_native_glob(b: &mut Criterion) {
  b.bench_function("native_glob", |b| b.iter(rs_native_glob));
}

criterion_group!(benches, glob_native_glob, bench_wax_glob);
criterion_main!(benches);

#[test]
fn have_same_result() {
  let res2 = rs_native_glob();
  let res3 = wax_glob();
  macro_rules! check_correctness {
      ($($res:expr),*) => {
        let mut v = Vec::new();
        $(
          println!("{}", $res.len());
          v.push($res.len());
        )*
        v.iter().reduce(|prev, curr| {assert_eq!(prev, curr); curr});
      };
  }

  check_correctness!(res1, res2, res3);
}
