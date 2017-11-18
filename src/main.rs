#[macro_use]
extern crate nom;

use nom::{IResult, ErrorKind};

#[derive(Debug, PartialEq)]
enum TokenKind {
    Number(f64),
    Plus,
}

#[derive(Debug, PartialEq)]
struct Token<'a> {
    literal: &'a str,
    start: usize,
    end: usize,
    kind: TokenKind,
}

named!(number<&[u8], TokenKind>, do_parse!(
  natural: fold_many1!(digit, 0.0, |acc: f64, item| (acc * 10.0) + f64::from(item)) >>
  float: opt!(
    do_parse!(
      tag!(".") >>
      res: fold_many1!(digit, (0.0, 1.0/10.0), |(acc, power), item| (acc + f64::from(item)*power, power/10.0)) >>
      (res.0)
    )
  ) >>
  ({
    let mut value = f64::from(natural);
    match float {
      Some(f) => value += f,
      _ => {},
    }
    TokenKind::Number(value)
  })
));

named!(digit<&[u8], u8>, alt!(
  map!(tag!("0"), |_| 0) |
  map!(tag!("1"), |_| 1) |
  map!(tag!("2"), |_| 2) |
  map!(tag!("3"), |_| 3) |
  map!(tag!("4"), |_| 4) |
  map!(tag!("5"), |_| 5) |
  map!(tag!("6"), |_| 6) |
  map!(tag!("7"), |_| 7) |
  map!(tag!("8"), |_| 8) |
  map!(tag!("9"), |_| 9)
));

named!(plus<&[u8], TokenKind>, map!(tag!("+"), |_| TokenKind::Plus));

fn eof<T>(input: &[T]) -> IResult<&[T], &[T]> {
    if input.len() == 0 {
        IResult::Done(&input[..], &input[..])
    } else {
        IResult::Error(ErrorKind::Eof)
    }
}

named!(lexer<&[u8], Vec<TokenKind>>, do_parse!(
  res: many0!(alt!(
    plus | number
  )) >>
  eof >>
  (res)
));

#[test]
fn first_test() {
    let input = "12.25+75.2".as_bytes();
    assert_eq!(
        lexer(&input),
        IResult::Done(&[][..], vec![
            TokenKind::Number(12.25),
            TokenKind::Plus,
            TokenKind::Number(75.2),
        ])
    );
}

fn main() {
    unimplemented!()
}
