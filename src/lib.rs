use std::{
  env,
  error::Error,
  collections::HashMap,
  process::Command,
};

#[derive(Debug, PartialEq)]
pub struct Config {
  query: String,
  is_new_commit: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("You should type number of task"),
    };

    let is_new_commit = env::var("NEW_COMMIT").is_ok() || match args.next() {
      Some(arg) => arg == "-n",
      None => false,
    };

    Ok(Config { query, is_new_commit })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // TODO: replace mock on request in TFS
  let mut mock = HashMap::new();
  mock.insert("555123".to_string(), "feature/TFS-555123 [FE]: some commit");

  let Config { query, is_new_commit } = config;

  let _ = Command::new("git")
    .args(["add", "src"])
    .output()
    .expect("Failed add to index");

  let mut commit_command = vec!["commit".to_string()];
  let mut push_command = vec!["push".to_string(), "origin".to_string(), format!("origin feature/TFS-{}", query)];

  if is_new_commit {
    let message = format!("-m {}", mock.get(&query).unwrap());
    commit_command.push(message);
  } else {
    commit_command.push("--amend".to_string());
    commit_command.push("--no-edit".to_string());
    push_command.push("-f".to_string());
  }

  let _ = Command::new("git").args(commit_command).output().unwrap();
  let _ = Command::new("git").args(push_command).output().unwrap();

  // if is_new_commit {
  //   let commit_message = format!("-m {}", mock.get(&query).unwrap());
  //   let _ = Command::new("git")
  //     .args(["commit".to_string(), commit_message])
  //     .output()
  //     .expect("Failed create commit");
  // } else {
  //   let _ = Command::new("git")
  //     .args(["commit", "--amend", "--no-edit"])
  //     .status()
  //     .expect("Failed write to commit");
  // }

  // let _ = Command::new("git")
  //   .args(["push"])
  //   .status()
  //   .expect("Failed push into repo");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_config() {
    let mut args = vec![String::from("/some/path"), String::from("555123")].into_iter();
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => "nothing".to_string(),
    };
    let is_new_commit = env::var("NEW_COMMIT").is_ok() || match args.next() {
      Some(arg) => arg == "-n",
      None => false,
    };

    assert_eq!(Ok(Config { query, is_new_commit }), Config::build(args));
  }
}