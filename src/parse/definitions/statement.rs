use super::delete::DeleteStatement;
use super::expression::Expression;
use super::insert::InsertStatement;
use super::select::SelectStatement;
use super::update::UpdateStatement;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Statement<'a> {
    Delete(DeleteStatement<'a>),
    Expression(Expression<'a>),
    Insert(InsertStatement<'a>),
    Select(SelectStatement<'a>),
    Update(UpdateStatement<'a>),
}
