#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod parse;
mod arg_config;

use std::env::args;
use std::process::exit;
use arg_config::{ArgConfig, ArgConfigParseError};

fn main() {
    let mut config = match ArgConfig::new(args()) {
        Ok(c) => c,
        Err(e) => {
            match e {
                ArgConfigParseError::NeedsHelpScreen => {
                    println!("{}", include_str!("help.txt"));
                    exit(0);
                }
                ArgConfigParseError::IOError(e) => {
                    println!("Error while reading command-line arguments: {}", e);
                    exit(1);
                },
            }
        },
    };
    let input = match config.read_input(1024) {
        Ok(i) => i,
        Err(e) => {
            println!("Error while reading input: {}", e);
            exit(2);
        },
    };
    let stmt = match parse::statement(&input) {
        Ok(s) => s,
        Err(e) => {
            println!("Error while parsing input: {}", e);
            exit(3);
        },
    };
    let yaml = match serde_yaml::to_string(&stmt) {
        Ok(y) => y,
        Err(e) => {
            println!("Error while serializing into YAML: {}", e);
            exit(4);
        },
    };
    match write!(config.output, "{}", yaml) {
        Err(e) => {
            println!("Error while writing output: {}", e);
            exit(5);
        },
        _ => {},
    }
}
