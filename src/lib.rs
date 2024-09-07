use std::{error::Error, fs};
use std::env;
use colored::*;

// #[derive(PartialEq, Debug)]
// enum GrepOption {

// }
#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    option: Option<String>
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
}