#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;

mod parse;

use std::fmt::Debug;

#[inline]
fn stringify_err<T, E: Debug>(input: Result<T, E>) -> Result<T, String> {
    input.map_err(|e| format!("{:#?}", e))
}

use serde::Serialize;

#[inline]
fn to_yaml<T: Serialize, E: Debug>(input: Result<T, E>) -> Result<String, String> {
    stringify_err(input)
        .and_then(|ref expr|
            stringify_err(serde_yaml::to_string(expr))
        )
}

const EXPR: &'static str = "\n
    foo(\n
        2 + 2 * 2,\n
        brr\n
    ).bar\n
AND\n
    bzz\n
";

const SELECT: &'static str = "SELECT * FROM foo INNER JOIN bar ON a = b ORDER BY baz LIMIT 2";

fn main() {
    match to_yaml(parse::expression(EXPR)) {
        Ok(value) => println!("Expression: {}", value),
        Err(err) => println!("Got error: {}", err),
    }
    match to_yaml(parse::select(SELECT)) {
        Ok(value) => println!("Selection: {}", value),
        Err(err) => println!("Got error: {}", err),
    }
}
