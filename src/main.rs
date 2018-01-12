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

const UPDATE: &'static str = "UPDATE IGNORE foo \nSET bar = 1, baz = null";

const INSERT: &'static str = "INSERT INTO foo (a, bar, baz) VALUES (1, 2, null)";

const DELETE: &'static str = "DELETE FROM foo, bar WHERE boo.a = 1 OR foo.a = 2";

fn main() {
    match to_yaml(parse::expression(EXPR)) {
        Ok(value) => println!("Expression: {}", value),
        Err(err) => println!("Got error while parsing expression: {}", err),
    }
    match to_yaml(parse::select(SELECT)) {
        Ok(value) => println!("Selection: {}", value),
        Err(err) => println!("Got error while parsing selection: {}", err),
    }
    match to_yaml(parse::update(UPDATE)) {
        Ok(value) => println!("Updating: {}", value),
        Err(err) => println!("Got error while parsing updating: {}", err),
    }
    match to_yaml(parse::insert(INSERT)) {
        Ok(value) => println!("Insertion: {}", value),
        Err(err) => println!("Got error while parsing insertion: {}", err),
    }
    match to_yaml(parse::delete(DELETE)) {
        Ok(value) => println!("Deletion: {}", value),
        Err(err) => println!("Got error while parsing deletion: {}", err),
    }
}

// TODO Reading input from file
// TODO Writing output to file
// TODO Writing output to console
// TODO Printing help screen
