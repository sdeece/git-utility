use std::{
  env,
  error::Error,
  collections::HashMap,
  process::{Command},
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

  let commit_message = &format!("-m {}", mock.get(&query).unwrap())[..];
  let mut commit_command = vec!["commit"];
  let mut push_command = vec!["push"];

  if is_new_commit {
    commit_command.push(commit_message);
  } else {
    commit_command.push("--amend");
    commit_command.push("--no-edit");
    push_command.push("-f");
  }

  let _ = git_command(vec!["add", "src"]);
  let _ = git_command(commit_command);
  let _ = git_command(push_command);

  Ok(())
}

fn git_command(args: Vec<&str>) -> Result<(), Box<dyn Error>> {
  let status = Command::new("git").args(args).status()?;
  Ok(println!("{status}"))
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