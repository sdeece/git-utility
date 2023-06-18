use std::{
  env,
  error::Error,
  collections::HashMap,
  process::Command,
};

#[derive(Debug, PartialEq)]
pub struct Config {
  path: String,
  query: String,
  is_new_commit: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let path = args.next().unwrap();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("You should type number of task"),
    };

    let is_new_commit = env::var("NEW_COMMIT").is_ok() || match args.next() {
      Some(arg) => arg == "-n",
      None => false,
    };

    Ok(Config { path, query, is_new_commit })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // TODO: replace mock on request in TFS
  let mut mock = HashMap::new();
  mock.insert("555123".to_string(), "feature/TFS-555123 [FE]: some commit".to_string());

  let Config { path, query, is_new_commit } = config;
  println!("{path}");
  println!("{query}");
  println!("{is_new_commit}");

  let _  = Command::new("git")
    .args(["add", "src"])
    .output()
    .expect("Failed add to index");

  if is_new_commit {
    let commit_message = format!("-m {}", mock.get(&query).unwrap());
    let _ = Command::new("git")
      .args(["commit".to_string(), commit_message])
      .output()
      .expect("Failed create commit");
  } else {
    let _ = Command::new("git")
      .arg("--amend --no-edit")
      .output()
      .expect("Failed write to commit");
  }

  let _ = Command::new("git")
    .args(["push".to_string(), "origin".to_string(), format!("feature/TFS-{}", query)])
    .output()
    .expect("Failed push into repo");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_config() {
    let mut args = vec![String::from("/some/path"), String::from("555123")].into_iter();

    let path = args.next().unwrap();
    let query = match args.next() {
      Some(arg) => arg,
      None => "nothing".to_string(),
    };
    let is_new_commit = env::var("NEW_COMMIT").is_ok() || match args.next() {
      Some(arg) => arg == "-n",
      None => false,
    };

    assert_eq!(Ok(Config { path, query, is_new_commit }), Config::build(args));
  }
}