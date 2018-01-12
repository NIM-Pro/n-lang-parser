use super::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectDistinctMod {
    All,
    Distinct,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectResultSize {
    Unknown,
    Big,
    Small,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectModifiers {
    pub distinct: SelectDistinctMod,
    pub high_priority: bool,
    pub straight_join: bool,
    pub result_size: SelectResultSize,
    pub buffer_result: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectExpressions<'a> {
    All,
    List(Vec<SelectExpression<'a>>),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectExpression<'a> {
    pub expression: Expression<'a>,
    pub alias: Option<&'a str>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectJoinCondition<'a> {
    Expression(Expression<'a>),
    Using(Vec<&'a str>),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectJoinSide {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectSidedJoinInfo {
    pub side: SelectJoinSide,
    pub is_outer: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectJoin<'a> {
    Inner(SelectJoinCondition<'a>),
    Cross(Option<SelectJoinCondition<'a>>),
    Straight(Option<Expression<'a>>),
    Sided(SelectSidedJoinInfo, SelectJoinCondition<'a>),
    Natural(SelectSidedJoinInfo),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectSource<'a> {
    Table {
        name: &'a str,
        alias: &'a str,
        partitions: Option<Vec<&'a str>>,
    },
    Join {
        join: SelectJoin<'a>,
        left: Box<SelectSource<'a>>,
        right: Box<SelectSource<'a>>,
    },
    Query {
        statement: Box<SelectStatement<'a>>,
        alias: Option<&'a str>,
    },
}

impl<'a> SelectSource<'a> {
    pub fn make_join(join: SelectJoin<'a>, left: Self, right: Self) -> Self {
        SelectSource::Join {
            join,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn make_query(statement: SelectStatement<'a>, alias: Option<&'a str>) -> Self {
        SelectSource::Query {
            statement: Box::new(statement),
            alias,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectOrderType {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectOrderItem<'a> {
    pub expression: Expression<'a>,
    pub order_type: SelectOrderType,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectGroupByClause<'a> {
    pub items: Vec<SelectOrderItem<'a>>,
    pub with_rollup: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectLimit<'a> {
    pub count: Expression<'a>,
    pub offset: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SelectStatement<'a> {
    pub modifiers: SelectModifiers,
    pub expressions: SelectExpressions<'a>,
    pub source: SelectSource<'a>,
    pub where_clause: Option<Expression<'a>>,
    pub group_by_clause: Option<SelectGroupByClause<'a>>,
    pub having_clause: Option<Expression<'a>>,
    pub order_by_clause: Option<Vec<SelectOrderItem<'a>>>,
    pub limit_clause: Option<SelectLimit<'a>>,
}
