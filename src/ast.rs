#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstError;

pub type Identifier<'i> = data::Identifier<'i>;
pub type Lambda<'i> = data::Lambda<'i, data::NoData>;
pub type Expression<'i> = data::Expression<'i, data::NoData>;
pub type Application<'i> = data::Application<'i, data::NoData>;
pub type Assignment<'i> = data::Assignment<'i, data::NoData>;
pub type Program<'i> = data::Program<'i, data::NoData>;

pub mod data;
mod fmt;
mod maker;
pub mod checked;

pub use maker::*;
