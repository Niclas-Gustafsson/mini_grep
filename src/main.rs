use std::env;
use std::process;
use colored::*;
use mini_grep::Config;


fn main() {
    //collect args from user
    let args: Vec<String> = env::args().collect();
    println!(" Args{:?}", args);

    if args.len() == 2 && &args[1] == "-h" {
        //user asking for help. Render options in the terminal and exit program to be re-run
        Config::display_help_options();
        process::exit(0)
    }
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err.red());
        process::exit(1)
    });
    println!("Config {:?}", &config);
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
