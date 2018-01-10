#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;

mod parse;

use std::fmt::Debug;

#[inline]
fn stringify_err<T, E : Debug>(input: Result<T, E>) -> Result<T, String> {
    input.map_err(|e| format!("{:#?}", e))
}

const EXPR: &'static str = "\n
    foo(\n
        2 + 2 * 2,\n
        brr\n
    ).bar\n
AND\n
    bzz\n
";

fn expr_to_yaml(expression: &str) -> Result<String, String> {
    stringify_err(parse::expression(expression))
        .and_then(|ref expr|
            stringify_err(serde_yaml::to_string(expr))
        )
}

const SELECT: &'static str = "SELECT * \nFROM foo \nINNER JOIN bar \nON a = b";

fn select_to_yaml(query: &str) -> Result<String, String> {
    stringify_err(parse::select(query))
        .and_then(|ref expr|
            stringify_err(serde_yaml::to_string(expr))
        )
}

fn main() {
    match expr_to_yaml(EXPR) {
        Ok(value) => println!("Expression: {}", value),
        Err(err) => println!("Got error: {}", err),
    }
    match select_to_yaml(SELECT) {
        Ok(value) => println!("Selection: {}", value),
        Err(err) => println!("Got error: {}", err),
    }
}
