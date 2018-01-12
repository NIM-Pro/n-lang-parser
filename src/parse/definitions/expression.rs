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
    Primitive,
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
