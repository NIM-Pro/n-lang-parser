pub mod definitions;

mod test;

peg_file! grammar("grammar.peg");

#[allow(dead_code)]
pub fn select<'input>(input: &'input str) -> Result<definitions::select::SelectStatement, grammar::ParseError> {
    grammar::select_stmt(input)
}

#[allow(dead_code)]
pub fn expression<'input>(input: &'input str) -> Result<definitions::expression::Expression<'input>, grammar::ParseError> {
    grammar::expression(input)
}

#[allow(dead_code)]
pub fn update<'input>(input: &'input str) -> Result<definitions::update::UpdateStatement<'input>, grammar::ParseError> {
    grammar::update_stmt(input)
}

#[allow(dead_code)]
pub fn insert<'input>(input: &'input str) -> Result<definitions::insert::InsertStatement<'input>, grammar::ParseError> {
    grammar::insert_stmt(input)
}

#[allow(dead_code)]
pub fn delete<'input>(input: &'input str) -> Result<definitions::delete::DeleteStatement<'input>, grammar::ParseError> {
    grammar::delete_stmt(input)
}

#[allow(dead_code)]
pub fn statement<'input>(input: &'input str) -> Result<definitions::statement::Statement<'input>, grammar::ParseError> {
    grammar::statement(input)
}
