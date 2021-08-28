use crate::EvaluateError;

use {
    super::Evaluated,
    crate::{
        ast::{AstLiteral, BinaryOperator, DataType, UnaryOperator},
        data::{Literal, Value},
        result::Result,
    },
    std::{
        borrow::Cow,
        convert::{TryFrom, TryInto},
    },
};

pub fn literal(ast_literal: &AstLiteral) -> Result<Evaluated<'_>> {
    Literal::try_from(ast_literal).map(Evaluated::Literal)
}

pub fn typed_string<'a>(data_type: &'a DataType, value: Cow<'a, String>) -> Result<Evaluated<'a>> {
    let literal = Literal::Text(value);

    Value::try_from_literal(data_type, &literal).map(Evaluated::from)
}

pub fn binary_op<'a>(
    op: &BinaryOperator,
    l: Evaluated<'a>,
    r: Evaluated<'a>,
) -> Result<Evaluated<'a>> {
    macro_rules! cmp {
        ($expr: expr) => {
            Ok(Evaluated::from(Value::Bool($expr)))
        };
    }

    macro_rules! cond {
        (l $op: tt r) => {{
            let l: bool = l.try_into()?;
            let r: bool = r.try_into()?;
            let v = l $op r;

            Ok(Evaluated::from(Value::Bool(v)))
        }};
    }

    match op {
        BinaryOperator::Plus => l.add(&r),
        BinaryOperator::Minus => l.subtract(&r),
        BinaryOperator::Multiply => l.multiply(&r),
        BinaryOperator::Divide => l.divide(&r),
        BinaryOperator::Modulo => l.modulo(&r),
        BinaryOperator::StringConcat => l.concat(r),
        BinaryOperator::Eq => cmp!(l == r),
        BinaryOperator::NotEq => cmp!(l != r),
        BinaryOperator::Lt => cmp!(l < r),
        BinaryOperator::LtEq => cmp!(l <= r),
        BinaryOperator::Gt => cmp!(l > r),
        BinaryOperator::GtEq => cmp!(l >= r),
        BinaryOperator::And => cond!(l && r),
        BinaryOperator::Or => cond!(l || r),
        BinaryOperator::Like => l.like(r),
        BinaryOperator::NotLike => cmp!(l.like(r)? == Evaluated::Literal(Literal::Boolean(false))),
    }
}

pub fn unary_op<'a>(op: &UnaryOperator, v: Evaluated<'a>) -> Result<Evaluated<'a>> {
    match op {
        UnaryOperator::Plus => v.unary_plus(),
        UnaryOperator::Minus => v.unary_minus(),
        UnaryOperator::Not => v.try_into().map(|v: bool| Evaluated::from(Value::Bool(!v))),
    }
}

pub fn between<'a>(
    target: Evaluated<'a>,
    negated: bool,
    low: Evaluated<'a>,
    high: Evaluated<'a>,
) -> Result<Evaluated<'a>> {
    let v = low <= target && target <= high;
    let v = negated ^ v;

    Ok(Evaluated::from(Value::Bool(v)))
}

pub fn simple_case<'a>(
    operand: Evaluated<'a>,
    when_then: Vec<(Evaluated<'a>, Evaluated<'a>)>,
    else_result: Option<Evaluated<'a>>,
) -> Result<Evaluated<'a>> {
    for w in when_then.iter() {
        if w.0.eq(&operand) {
            return Ok(w.1.to_owned());
        }
    }
    match else_result {
        Some(result) => Ok(result),
        None => Ok(Evaluated::from(Value::Null)),
    }
}

pub fn searched_case<'a>(
    when_then: Vec<(Evaluated<'a>, Evaluated<'a>)>,
    else_result: Option<Evaluated<'a>>,
) -> Result<Evaluated<'a>> {
    for w in when_then.iter() {
        match w.0.to_owned().try_into()? {
            Value::Bool(v) => {
                if v {
                    return Ok(w.1.to_owned());
                }
            }
            _ => return Err(EvaluateError::BooleanTypeRequired("CASE".to_owned()).into()),
        };
    }
    match else_result {
        Some(result) => Ok(result),
        None => Ok(Evaluated::from(Value::Null)),
    }
}
