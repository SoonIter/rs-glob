pub use glob_match::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use std::{collections::HashSet, process::Command};
pub struct NativeGlobOptions {
  pub cwd: String,
}
pub fn native_glob(glob: &[&str], options: NativeGlobOptions) -> Vec<String> {
  let NativeGlobOptions { cwd } = options;

  globwalk::GlobWalkerBuilder::from_patterns(cwd, glob)
    .follow_links(true)
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
    .map(|x| x.path().to_str().unwrap().to_string())
    .collect()
}
pub async fn git_status() -> Vec<String> {
  let output = Command::new("git")
    .arg("status")
    .arg("-s")
    .arg("-u")
    .arg(".")
    .output()
    .expect("Failed to execute git command");

  let output_string = String::from_utf8(output.stdout).unwrap();
  let paths: Vec<String> = output_string
    .lines()
    .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
    .collect();

  paths
}

pub async fn git_ls_tree() -> Vec<String> {
  let output = Command::new("git")
    .arg("ls-tree")
    .arg("HEAD")
    .arg("-r")
    .output()
    .expect("Failed to execute git command");
  let output_string = String::from_utf8(output.stdout).unwrap();

  let paths: Vec<String> = output_string
    .lines()
    .map(|line| line.split_whitespace().nth(3).unwrap().to_string())
    .collect();

  paths
}
async fn git_ls_tree_wrapper(glob: &str) -> Vec<String> {
  glob_match_arr(glob, git_ls_tree().await).await
}
async fn git_status_wrapper(glob: &str) -> Vec<String> {
  glob_match_arr(glob, git_status().await).await
}

async fn glob_match_arr(glob: &str, paths: Vec<String>) -> Vec<String> {
  paths
    .par_iter()
    .filter_map(|item| {
      if glob_match(glob, item) {
        Some(item.to_owned())
      } else {
        None
      }
    })
    .collect()
}

pub async fn rg(glob: &str) -> Vec<String> {
  let (paths1, paths2) = futures::join!(git_ls_tree_wrapper(glob), git_status_wrapper(glob));
  let mut paths_set = HashSet::new();

  for path in paths1 {
    paths_set.insert(path);
  }
  for path in paths2 {
    paths_set.insert(path);
  }
  paths_set.iter().map(|i| i.to_string()).collect()
}
