use std::env;
use std::process;
mod minigrep_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Here are the arguments, {:?}", &args);

    let config = minigrep_config::Config::new(&args).unwrap_or_else(|err|{
        println!("There was an error processing your args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_config::run(config) {
        println!("Error in reading text. Error: {}", e);

        process::exit(1);
    }
}

