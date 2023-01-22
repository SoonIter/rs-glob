pub use glob_match::*;

use std::{collections::HashSet, process::Command};

pub fn git_status() -> Vec<String> {
  let output = Command::new("git")
    .arg("status")
    .arg("-s")
    .arg("-u")
    .arg(".")
    .output()
    .expect("Failed to execute git command");

  let output_string = String::from_utf8(output.stdout).unwrap();
  let paths: Vec<&str> = output_string
    .lines()
    .map(|line| line.split_whitespace().nth(1).unwrap())
    .collect();

  paths.iter().map(|s| s.to_string()).collect()
}

pub fn git_ls_tree() -> Vec<String> {
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

pub fn rg(pattern: &str) -> Vec<String> {
  let paths1 = git_ls_tree();
  let paths2 = git_status();
  let mut paths_set = HashSet::new();

  for path in paths1 {
    paths_set.insert(path);
  }
  for path in paths2 {
    paths_set.insert(path);
  }

  let x = paths_set
    .iter()
    .filter(|&s| glob_match(pattern, s))
    .map(|i| i.to_string())
    .collect();
  x
}
