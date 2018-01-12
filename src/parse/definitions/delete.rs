use super::select::SelectSource;
use super::select::SelectOrderItem;
use super::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DeleteStatement<'a> {
    pub low_priority: bool,
    pub quick: bool,
    pub ignore: bool,
    pub source: SelectSource<'a>,
    pub where_clause: Option<Expression<'a>>,
    pub order_by_clause: Option<Vec<SelectOrderItem<'a>>>,
    pub limit: Option<Expression<'a>>,
}
