
pub fn search<'a>(
  query: &str,
  contents: &'a str,
) -> impl Iterator<Item = &'a str> {
  contents
    .lines()
    .filter(move |l| l.contains(query))
}

pub fn case_insensitive_search<'a>(
  query: &str,
  contents: &'a str,
) -> impl Iterator<Item = &'a str> {
  contents.
    lines().
    filter(move |l| l.to_lowercase().contains(&query.to_lowercase()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; 
  
    let res: Vec<&str> = search(query, contents).collect();
    assert_eq!(vec!["safe, fast, productive."], res)
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let res: Vec<&str> = case_insensitive_search(query, contents).collect();
    
    assert_eq!(vec!["Rust:", "Trust me."], res)
  }
}