use std::collections::BTreeSet;
use super::data;

#[derive(Debug, Clone)]
pub struct LambdaData<'i> {
    pub id: usize,
    pub captures: BTreeSet<Identifier<'i>>
}

#[derive(Debug, Clone, Copy)]
pub struct CheckData;

impl<'i> data::ASTData<'i> for CheckData {
    type LambdaData = LambdaData<'i>;
}

pub use data::Identifier;
pub type Lambda<'i> = data::Lambda<'i, CheckData>;
pub type Expression<'i> = data::Expression<'i, CheckData>;
pub type Application<'i> = data::Application<'i, CheckData>;
pub type Assignment<'i> = data::Assignment<'i, CheckData>;
pub type Program<'i> = data::Program<'i, CheckData>;
