use crate::ast::checked::*;

pub trait CodegenTarget {
    fn generate(&self, program: &Program<'_>) -> String;
}

mod util;
mod javascript;
mod python;
mod cplusplus;

pub use javascript::JavaScript;
pub use python::Python;
pub use cplusplus::CPlusPlus;
