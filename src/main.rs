use std::{env, process};
use git_process::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {err}");
        process::exit(-1);
    });

    if let Err(err) = git_process::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
