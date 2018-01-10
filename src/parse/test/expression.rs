#[allow(unused_imports)]
use super::super::expression;

#[allow(unused_imports)]
use super::super::definitions::*;

#[test]
fn simple_expression() {
    assert_eq!(
        expression("2.25+-0x2F"),
        Ok(Expression::make_binary(
            BinaryOperator::Plus,
            Expression::Literal(LiteralType::Number, "2.25"),
            Expression::make_prefix_unary(
                PrefixUnaryOperator::Minus,
                Expression::Literal(LiteralType::Number, "0x2F"),
            ),
        ))
    );
}

#[test]
fn call_and_appeal_expression() {
    assert_eq!(
        expression("foo(bar, 2.3, (false, 9)).bar"),
        Ok(Expression::make_postfix_unary(
            PostfixUnaryOperator::FieldAppeal("bar"),
            Expression::FunctionCall(
                "foo",
                vec!(
                    Expression::Identifier("bar"),
                    Expression::Literal(LiteralType::Number, "2.3"),
                    Expression::Set(vec!(
                        Expression::Literal(LiteralType::Boolean, "false"),
                        Expression::Literal(LiteralType::Number, "9"),
                    )),
                ),
            ),
        ))
    );
}

#[test]
fn boolean_expression() {
    assert_eq!(
        expression("foo > 3 AND bar IS NOT NULL"),
        Ok(Expression::make_binary(
            BinaryOperator::And,
            Expression::make_binary(
                BinaryOperator::MoreThan,
                Expression::Identifier("foo"),
                Expression::Literal(LiteralType::Number, "3"),
            ),
            Expression::make_prefix_unary(
                PrefixUnaryOperator::Not,
                Expression::make_postfix_unary(
                    PostfixUnaryOperator::IsNull,
                    Expression::Identifier("bar"),
                ),
            ),
        ))
    );
}