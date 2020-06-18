#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstError;

pub type Identifier<'i> = &'i str;

#[derive(Clone, PartialEq, Eq)]
pub struct Lambda<'i> {
    pub argument: Identifier<'i>,
    pub body: Application<'i>
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expression<'i> {
    Lambda(Lambda<'i>),
    Parenthesis(Application<'i>),
    Identifier(Identifier<'i>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Application<'i> {
    pub expressions: Vec<Expression<'i>>
}

#[derive(Clone, PartialEq, Eq)]
pub struct Assignment<'i> {
    pub target: Identifier<'i>,
    pub value: Application<'i>
}

#[derive(Clone, PartialEq, Eq)]
pub struct Program<'i> {
    pub assignments: Vec<Assignment<'i>>
}

mod fmt;
mod maker;

pub use maker::*;
