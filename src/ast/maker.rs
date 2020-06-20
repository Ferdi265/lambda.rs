use std::rc::Rc;

use super::nodata::*;

use crate::error::Error;
use crate::parser::Rule;
use crate::parser::Pair;
use crate::parser::Pairs;

pub trait Maker<'i, T>: FnOnce(Pair<'i>) -> Result<T, Error> {}
impl<'i, T, F> Maker<'i, T> for F where F: FnOnce(Pair<'i>) -> Result<T, Error> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstMakeError;

fn ast_error() -> Error {
    Error::AstMakeError(AstMakeError)
}

fn ast_error_result<T>() -> Result<T, Error> {
    Err(ast_error())
}

fn ensure_rule(pair: &Pair<'_>, rule: Rule) -> Result<(), Error> {
    if pair.as_rule() != rule { ast_error_result()? }
    Ok(())
}

pub fn make_identifier(pair: Pair<'_>) -> Result<Identifier<'_>, Error> {
    ensure_rule(&pair, Rule::identifier)?;

    let ident = pair.as_str();

    let mut inner = pair.into_inner();
    if inner.next() != None { ast_error_result()? }

    Ok(ident)
}

pub fn make_rc_lambda(pair: Pair<'_>) -> Result<Rc<Lambda<'_>>, Error> {
    ensure_rule(&pair, Rule::lambda)?;

    let mut inner = pair.into_inner();
    let ident = inner.next().ok_or_else(ast_error)?;
    let expr = inner.next().ok_or_else(ast_error)?;
    if inner.next() != None { ast_error_result()? }

    Ok(Rc::new(Lambda {
        argument: make_identifier(ident)?,
        body: make_rc_application(expr)?,
        data: ()
    }))
}

pub fn make_lambda(pair: Pair<'_>) -> Result<Lambda<'_>, Error> {
    make_rc_lambda(pair).map(|app| app.as_ref().clone())
}

pub fn make_rc_parenthesis(pair: Pair<'_>) -> Result<Rc<Application<'_>>, Error> {
    ensure_rule(&pair, Rule::parenthesis)?;

    let mut inner = pair.into_inner();
    let app = inner.next().ok_or_else(ast_error)?;
    if inner.next() != None { ast_error_result()? }

    make_rc_application(app)
}

pub fn make_parenthesis(pair: Pair<'_>) -> Result<Application<'_>, Error> {
    make_rc_parenthesis(pair).map(|app| app.as_ref().clone())
}

pub fn make_expression(pair: Pair<'_>) -> Result<Expression<'_>, Error> {
    ensure_rule(&pair, Rule::expression)?;

    let mut inner = pair.into_inner();
    let expr = inner.next().ok_or_else(ast_error)?;
    if inner.next() != None { ast_error_result()? }

    match expr.as_rule() {
        Rule::lambda => make_rc_lambda(expr).map(Expression::Lambda),
        Rule::parenthesis => make_rc_parenthesis(expr).map(Expression::Parenthesis),
        Rule::identifier => make_identifier(expr).map(Expression::Identifier),
        _ => ast_error_result()
    }
}

pub fn make_rc_application(pair: Pair<'_>) -> Result<Rc<Application<'_>>, Error> {
    ensure_rule(&pair, Rule::application)?;

    let exprs: Vec<_> = pair.into_inner()
        .map(make_expression)
        .collect::<Result<Vec<_>, _>>()?;

    let app = exprs.into_iter()
        .rev()
        .fold(None, |tail, head| Some(Rc::new(Application {
            head,
            tail,
            data: ()
        })));

    app.ok_or_else(ast_error)
}

pub fn make_application(pair: Pair<'_>) -> Result<Application<'_>, Error> {
    make_rc_application(pair).map(|app| app.as_ref().clone())
}

pub fn make_assignment(pair: Pair<'_>) -> Result<Assignment<'_>, Error> {
    ensure_rule(&pair, Rule::assignment)?;

    let mut inner = pair.into_inner();
    let ident = inner.next().ok_or_else(ast_error)?;
    let app = inner.next().ok_or_else(ast_error)?;
    if inner.next() != None { ast_error_result()? }

    Ok(Assignment {
        target: make_identifier(ident)?,
        value: make_rc_application(app)?,
        data: ()
    })
}

pub fn make_program(pair: Pair<'_>) -> Result<Program<'_>, Error> {
    ensure_rule(&pair, Rule::program)?;

    let asss: Vec<_> = pair.into_inner()
        .flat_map(|ass| match ass.as_rule() {
            Rule::assignment => Some(make_assignment(ass)),
            Rule::EOI => None,
            _ => Some(ast_error_result())

        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Program {
        assignments: asss,
        data: ()
    })
}

pub fn from_pairs<'i, T, M>(mut pairs: Pairs<'i>, maker: M) -> Result<T, Error>
    where T: 'i, M: Maker<'i, T>
{
    let pair = pairs.next().ok_or_else(ast_error)?;
    if pairs.next() != None { ast_error_result()? }

    maker(pair)
}
