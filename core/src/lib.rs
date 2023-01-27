pub use glob_match::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use std::{collections::HashSet, process::Command};

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
async fn git_ls_tree_wrapper(pattern: &str) -> Vec<String> {
  glob_match_arr(pattern, git_ls_tree().await).await
}
async fn git_status_wrapper(pattern: &str) -> Vec<String> {
  glob_match_arr(pattern, git_status().await).await
}

async fn glob_match_arr(pattern: &str, arr: Vec<String>) -> Vec<String> {
  arr
    .par_iter()
    .filter_map(|item| {
      if glob_match(pattern, item) {
        Some(item.to_owned())
      } else {
        None
      }
    })
    .collect()
}

pub async fn rg(pattern: &str) -> Vec<String> {
  let (paths1, paths2) = futures::join!(git_ls_tree_wrapper(pattern), git_status_wrapper(pattern));
  let mut paths_set = HashSet::new();

  for path in paths1 {
    paths_set.insert(path);
  }
  for path in paths2 {
    paths_set.insert(path);
  }
  paths_set.iter().map(|i| i.to_string()).collect()
}
