use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {panic!("Invalid number of arguments passed: expecting 2");}

    let cfg = Config::new(&args);

    let Ok(it) = read_lines(cfg.file_path) else {
        todo!("handle IO errors");
    };

    for line in it {
        println!("{}", line.unwrap());
    }
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

struct Config<'a> {
    // query:      &'a str,
    file_path:  &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Config<'a> {
        // let query     = &args[1];
        let file_path = &args[2];

        Config { /*query*/ file_path }
    }
}
