
use std::{fs, env};
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    // parser cl arguments
    let config = Config::build(&args).
        unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    // error handling
    if let Err(err) = run(&config) {
        println!("Application error: {err}");
        process::exit(1);
    };
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config<'a> {
    query:      &'a str,
    file_path:  &'a str,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        let len = args.len();
        if len < 3 {return Err("not enough arguments");}

        let query     = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}