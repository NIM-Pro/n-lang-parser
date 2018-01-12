use std::io::{Read, Result as IOResult, Error as IOError, stdin, stdout, Write};
use std::env::Args;
use std::fs::File;
use std::path::Path;

pub struct ArgConfig {
    pub input: Box<Read>,
    pub output: Box<Write>,
}

pub enum ArgConfigParseError {
    NeedsHelpScreen,
    IOError(IOError),
}

impl ArgConfig {
    pub fn new(mut args: Args) -> Result<Self, ArgConfigParseError> {
        args.next();
        let input: Box<Read> = match args.next() {
            Some(ref filename) => {
                if filename.trim() == "con" {
                    Box::new(stdin())
                } else {
                    match File::open(Path::new(filename)) {
                        Ok(file) => Box::new(file),
                        Err(e) => return Err(ArgConfigParseError::IOError(e)),
                    }
                }
            },
            None => return Err(ArgConfigParseError::NeedsHelpScreen),
        };
        let output: Box<Write> = match args.next() {
            Some(ref filename) => {
                if filename.trim() == "con" {
                    Box::new(stdout())
                } else {
                    match File::create(Path::new(filename)) {
                        Ok(file) => Box::new(file),
                        Err(e) => return Err(ArgConfigParseError::IOError(e)),
                    }
                }
            },
            None => Box::new(stdout()),
        };
        Ok(Self {
            input,
            output,
        })
    }
    pub fn read_input(&mut self, estimated_size: usize) -> IOResult<String> {
        let mut result = String::with_capacity(estimated_size);
        if let Err(e) = self.input.read_to_string(&mut result) {
            return Err(e)
        }
        result.shrink_to_fit();
        Ok(result)
    }
}