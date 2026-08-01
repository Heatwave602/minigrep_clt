
use std::{fs, env};
use std::process;
use std::error::Error;
use minigrep::{case_insensitive_search, search};

fn main() {
    // parser cl arguments
    let config = Config::build(env::args()).
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
    let contents = fs::read_to_string(&config.file_path)?;
    
    let lines = if config.ignore_case {
        case_insensitive_search(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query:      String,
    file_path:  String,
    ignore_case: bool,
}

impl<'a> Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("not enough arguments"),
        };
        let file_path = match args.next() {
            Some(p) => p,
            None => return Err("not enough arguments"),
        };

        // let len = args.len();
        // if len < 3 {return Err("not enough arguments");}

        // let query     = &args[1];
        // let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let case_sensitive = !env::var("CASE_SENSITIVE").is_ok();
        let ignore_case = 
        (ignore_case || 
        match args.next() {
            Some(arg) => arg == "-ic",
            None => false,
        }) && case_sensitive;
        
        Ok(Config { query, file_path, ignore_case})
    }
}