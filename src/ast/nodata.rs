use super::generic;

#[derive(Debug, Clone, Copy)]
pub struct NoData;
impl generic::ASTData<'_> for NoData {}

pub use generic::Identifier;
pub type Lambda<'i> = generic::Lambda<'i, NoData>;
pub type Expression<'i> = generic::Expression<'i, NoData>;
pub type Application<'i> = generic::Application<'i, NoData>;
pub type Assignment<'i> = generic::Assignment<'i, NoData>;
pub type Program<'i> = generic::Program<'i, NoData>;
