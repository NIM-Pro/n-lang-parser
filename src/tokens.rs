#[derive(Debug, PartialEq)]
pub struct Number<'a>(pub usize, pub &'a str);
