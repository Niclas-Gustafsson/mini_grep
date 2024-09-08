use std::{error::Error, fs};
// use std::env;
use colored::*;

// #[derive(PartialEq, Debug)]
// enum GrepOption {

// }
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub option: Option<String>
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let mut results: Option<Vec<&str>> = None;
    let mut results_with_lines: Option<Vec<String>> = None;

    if let Some(ref option) = config.option {
       results =  match option.as_str() {
            "-c" => Some(config.search_case_sensitive(&file_content)), 
            "-i" => Some(config.search_case_insensitive(&file_content)), 
          
            _ =>  None,
        };

        results_with_lines =  match option.as_str() {
            "-l" => {
                let res = config.search_with_line_number(&file_content);
               Some(res)
            },
            _ =>  None,
        };
        //Value from result moved to results_to_print. Results is dropped after this
        // results_to_print = results;
    //    results
    }

  
    if let Some(results) = results {

        if results.len() <= 0 {
            let message ="No results found"; 
            println!("{}", message.red());
        }
        for line in results {
            println!("{}", line.green());
        }
    }
  
    if let Some(results_with_lines) = results_with_lines {

        if results_with_lines.len() <= 1 {
            let message ="No results found"; 
            println!("{}", message.red());
        }
        for line in results_with_lines {
            println!("{}", line.green());
        }
    }

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        println!("{:?}", args);
        //if option is provided make Config.option(Some(String))
        if args.len() == 4 {
            let option = args[1].clone();
            let query = args[2].clone();
            let filename = args[3].clone();
             Ok(Self {query, filename, option: Some(option)})
        } else if args.len() == 3 {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Self {query, filename, option: None})
        } else {
            return Err("Not enough arguments, try using the '-h' for help.");
        }
    }

/*     pub fn parse_option(&self) {
        //ref dereferences the value inside of the Option, borrowing the value instead of moving it.
        if let Some(ref option) = self.option {
            match option.as_str() {
                "-c" => println!("Case sensitive chosen"), 
                "-i" => println!("Case insensitive chosen"), 
                "-l" => println!("Show lines option"),
                _ => println!("Invalid option"),
            }
        } else {
            println!("No option provided");
        }
    } */

    fn search_case_sensitive <'a>(&self, content: &'a str) -> Vec<&'a str>{
        let mut results = Vec::new();

        for line in content.lines() {
            if line.contains(&self.query) {
                results.push(line.trim());
            }
        }
        results
    }

    fn search_case_insensitive<'a>(&self, content: &'a str) -> Vec<&'a str>{
        let query = self.query.to_lowercase();
        let mut results = Vec::new();
    
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line.trim());
            }
        }
        results
    }

    fn search_with_line_number (&self, content: & str) -> Vec<String>{
        let mut results = Vec::new();
        
        for line in content.lines() {
            if line.contains(&self.query) {
                results.push(line.trim().to_string());
            }
        }
        if results.len() > 0 {

            let line_count = format!("Lines: {}", results.len());
      
            results.insert(0, line_count);
        }
        results
    }


    pub fn display_help_options() {
        let option = "-i".yellow();
        let query = "tomatoes".yellow();
        let filename = "recipe.txt".yellow();

        println!("--------------------| OPTIONS |--------------------");
        println!("Basic usage:");
        println!("[path-to-executable] [OPTION] [QUERY] [FILENAME]   |");
        println!("Example: ./mini_grep.exe {} {} {}", option, query, filename );
        println!("\n");
        println!("| CMD       | DESC                                 |");
        println!("| -h        Displays help list                     |");
        println!("| -c        Makes search case-sesnsitive.          |");
        println!("| -i        Makes search case-insensitive          |");
        println!("| -l        Returns all the lines containing query |");
    }
}