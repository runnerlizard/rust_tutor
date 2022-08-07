use std::{ process, env };
use minigrep::{ run, Config };

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}

