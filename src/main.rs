use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Here are the arguments, {:?}", &args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("There was an error processing your args: {}", err);
        process::exit(1);
    });

    let source_contents = fs::read_to_string(config.search_source_file)
        .expect("Your file could not be read");


}



struct Config {
    search_string: String,
    search_source_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("There are not enough arguments");
        }

        let search_string = args[1].clone();
        let search_source_file = args[2].clone();

        Ok(Config{search_string,search_source_file})
    }
}
