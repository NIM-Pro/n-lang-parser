use super::expression::Expression;
use super::select::SelectOrderItem;
use super::select::SelectSource;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum UpdateValue<'a> {
    Default,
    Expression(Expression<'a>),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct UpdateAssignment<'a> {
    pub column: Expression<'a>,
    pub value: UpdateValue<'a>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum UpdateType<'a> {
    Simple {
        source: SelectSource<'a>,
        order_by_clause: Option<Vec<SelectOrderItem<'a>>>,
        limit: Option<Expression<'a>>,
    },
    MultiTable {
        source: Vec<SelectSource<'a>>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct UpdateStatement<'a> {
    pub low_priority: bool,
    pub ignore: bool,
    pub update: UpdateType<'a>,
    pub assignments: Vec<UpdateAssignment<'a>>,
    pub where_clause: Option<Expression<'a>>,
}