
use std::{fs, env};
use std::process;
use std::error::Error;
use minigrep::{case_insensitive_search, search};

fn main() {
    let args: Vec<String> = env::args().collect();

    // parser cl arguments
    let config = Config::build(&args).
        unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    // error handling
    if let Err(err) = run(&config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let lines = if config.ignore_case {
        case_insensitive_search(config.query, &contents)
    }
    else {
        search(config.query, &contents)
    };
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

struct Config<'a> {
    query:      &'a str,
    file_path:  &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        let len = args.len();
        if len < 3 {return Err("not enough arguments");}

        let query     = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}