#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectDistinctMod {
    All,
    Distinct,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SelectResultSize {
    Big,
    Small,
    Unknown,
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
    Cross(SelectJoinCondition<'a>),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PrefixUnaryOperator {
    // Logical operators
    Not,
    // Set operators
    All,
    Any,
    // Arithmetic operators
    Plus,
    Minus,
    Tilde,
    // Language specific operators
    Binary,
    Row,
    Exists,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PostfixUnaryOperator<'a> {
    // Primitive comparison operators
    IsNull,
    IsTrue,
    IsFalse,
    IsUnknown,
    FieldAppeal(&'a str),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BinaryOperator {
    // Logical operators
    Or,
    XOr,
    And,
    // Bit operators
    BitOr,
    BitXOr,
    BitAnd,
    ShiftLeft,
    ShiftRight,
    // Set operators
    IsIn,
    // Comparison operators
    Equals,
    MoreThanOrEquals,
    MoreThan,
    LessThanOrEquals,
    LessThan,
    Like,
    SoundsLike,
    RegExp,
    // Arithmetic operators
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Div,
    // Language specific operators
    Collate,
    Interval,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LiteralType {
    Number,
    String,
    Character,
    Boolean,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Expression<'a> {
    Literal {
        literal_type: LiteralType,
        text: &'a str,
    },
    Identifier(&'a str),
    Set(Vec<Expression<'a>>),
    FunctionCall {
        name: &'a str,
        arguments: Vec<Expression<'a>>,
    },
    PrefixUnaryExpression {
        operator: PrefixUnaryOperator,
        expression: Box<Expression<'a>>,
    },
    PostfixUnaryExpression {
        operator: PostfixUnaryOperator<'a>,
        expression: Box<Expression<'a>>
    },
    BinaryExpression {
        operator: BinaryOperator,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
}

impl<'a> Expression<'a> {
    pub fn make_prefix_unary(operator: PrefixUnaryOperator, expr: Self) -> Self {
        Expression::PrefixUnaryExpression {
            operator,
            expression: Box::new(expr),
        }
    }
    pub fn make_postfix_unary(operator: PostfixUnaryOperator<'a>, expr: Self) -> Self {
        Expression::PostfixUnaryExpression {
            operator,
            expression: Box::new(expr),
        }
    }
    pub fn make_binary(operator: BinaryOperator, left: Self, right: Self) -> Self {
        Expression::BinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn make_function_call(name: &'a str, arguments: Vec<Expression<'a>>) -> Self {
        Expression::FunctionCall {
            name,
            arguments,
        }
    }
    pub fn make_literal(literal_type: LiteralType, text: &'a str) -> Self {
        Expression::Literal {
            literal_type,
            text,
        }
    }
}
