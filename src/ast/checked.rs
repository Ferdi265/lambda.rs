use std::collections::BTreeSet;

pub use super::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda<'i> {
    pub id: usize,
    pub argument: Identifier<'i>,
    pub body: Application<'i>,
    pub captures: BTreeSet<Identifier<'i>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'i> {
    Lambda(Lambda<'i>),
    Parenthesis(Application<'i>),
    Identifier(Identifier<'i>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'i> {
    pub expressions: Vec<Expression<'i>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<'i> {
    pub target: Identifier<'i>,
    pub value: Application<'i>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'i> {
    pub assignments: Vec<Assignment<'i>>
}
