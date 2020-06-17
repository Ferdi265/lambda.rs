#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstError;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'i>(pub &'i str);

#[derive(Clone, PartialEq, Eq)]
pub struct Lambda<'i>(pub Identifier<'i>, pub Application<'i>);

#[derive(Clone, PartialEq, Eq)]
pub enum Expression<'i> {
    Lambda(Lambda<'i>),
    Parenthesis(Application<'i>),
    Identifier(Identifier<'i>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Application<'i>(pub Vec<Expression<'i>>);

#[derive(Clone, PartialEq, Eq)]
pub struct Assignment<'i>(pub Identifier<'i>, pub Application<'i>);

#[derive(Clone, PartialEq, Eq)]
pub struct Program<'i>(pub Vec<Assignment<'i>>);

mod fmt;
mod maker;

pub use maker::*;
