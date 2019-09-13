use std::error::Error;
use std::fs;

pub struct Config {
    pub search_string: String,
    pub search_source_file: String,
    }

impl Config {
     pub fn new(args: &[String]) -> Result<Config, &'static str>{
         if args.len() < 3{
             return Err("There are not enough arguments");
         }
         let search_string = args[1].clone();
         let search_source_file = args[2].clone();
         Ok(Config{search_string,search_source_file})
     }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let source_contents = fs::read_to_string(config.search_source_file)
        .expect("Your file could not be read");
    let result = search(&config.search_string, &source_contents);
    for line in result{
        println!("{}", line)
    }
    Ok(())
}


fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test(){
        let query = "spicy";
        let contents = "Rust is honestly \nbig spicy";

        assert_eq!(
            vec!["big spicy"],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive(){
        let query = "big";
        let contents = "big SPiCy\n Bigger spicy";

        assert_eq!(
            vec!["big SPiCy"],
            search(&query, &contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "big";
        let contents = "big SPiCy\n Bigger spicy";

        assert_eq!(
            vec!["big SPiCy", " Bigger spicy"],
            search_insensitive(&query, &contents)
        );
    }
}


