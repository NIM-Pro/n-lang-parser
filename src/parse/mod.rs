pub mod definitions;

mod test;

peg_file! grammar("grammar.peg");

pub fn select<'input>(input: &'input str) -> Result<definitions::SelectStatement, grammar::ParseError> {
    grammar::select_stmt(input)
}

pub fn expression<'input>(input: &'input str) -> Result<definitions::Expression<'input>, grammar::ParseError> {
    grammar::expression(input)
}
