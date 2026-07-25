use std::io;
use std::fs::File;

pub fn search<'a>(query: &str, lines: &'a io::Lines<io::BufReader<File>>) -> Vec<&'a str> {
  unimplemented!();
}