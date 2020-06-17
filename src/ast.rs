use crate::error::Error;

use crate::parser::Rule;
use crate::parser::Pair;
use crate::parser::Pairs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'i>(pub &'i str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda<'i>(pub Identifier<'i>, pub Application<'i>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'i> {
    Lambda(Lambda<'i>),
    Parenthesis(Application<'i>),
    Identifier(Identifier<'i>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'i>(pub Vec<Expression<'i>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<'i>(pub Identifier<'i>, pub Application<'i>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'i>(pub Vec<Assignment<'i>>);

pub trait Maker<'i, T>: FnOnce(Pair<'i>) -> Result<T, Error> {}
impl<'i, T, F> Maker<'i, T> for F where F: FnOnce(Pair<'i>) -> Result<T, Error> {}

fn ast_error() -> Error {
    Error::AstError(AstError)
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

    Ok(Identifier(ident))
}

pub fn make_lambda(pair: Pair<'_>) -> Result<Lambda<'_>, Error> {
    ensure_rule(&pair, Rule::lambda)?;

    let mut inner = pair.into_inner();
    let ident = inner.next().ok_or(ast_error())?;
    let expr = inner.next().ok_or(ast_error())?;
    if inner.next() != None { ast_error_result()? }

    Ok(Lambda(make_identifier(ident)?, make_application(expr)?))
}

pub fn make_parenthesis(pair: Pair<'_>) -> Result<Application<'_>, Error> {
    ensure_rule(&pair, Rule::parenthesis)?;

    let mut inner = pair.into_inner();
    let app = inner.next().ok_or(ast_error())?;
    if inner.next() != None { ast_error_result()? }

    make_application(app)
}

pub fn make_expression(pair: Pair<'_>) -> Result<Expression<'_>, Error> {
    ensure_rule(&pair, Rule::expression)?;

    let mut inner = pair.into_inner();
    let expr = inner.next().ok_or(ast_error())?;
    if inner.next() != None { ast_error_result()? }

    match expr.as_rule() {
        Rule::lambda => make_lambda(expr).map(Expression::Lambda),
        Rule::parenthesis => make_parenthesis(expr).map(Expression::Parenthesis),
        Rule::identifier => make_identifier(expr).map(Expression::Identifier),
        _ => ast_error_result()
    }
}

pub fn make_application(pair: Pair<'_>) -> Result<Application<'_>, Error> {
    ensure_rule(&pair, Rule::application)?;

    let exprs: Vec<_> = pair.into_inner()
        .map(make_expression)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Application(exprs))
}

pub fn make_assignment(pair: Pair<'_>) -> Result<Assignment<'_>, Error> {
    ensure_rule(&pair, Rule::assignment)?;

    let mut inner = pair.into_inner();
    let ident = inner.next().ok_or(ast_error())?;
    let app = inner.next().ok_or(ast_error())?;
    if inner.next() != None { ast_error_result()? }

    Ok(Assignment(make_identifier(ident)?, make_application(app)?))
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

    Ok(Program(asss))
}

pub fn from_pairs<'i, T, M>(mut pairs: Pairs<'i>, maker: M) -> Result<T, Error>
    where T: 'i, M: Maker<'i, T>
{
    let pair = pairs.next().ok_or(ast_error())?;
    if pairs.next() != None { ast_error_result()? }

    maker(pair)
}
