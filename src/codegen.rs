use crate::ast::checked::*;

pub trait CodegenTarget {
    fn generate(&self, program: &Program<'_>) -> String;
}

mod javascript;
mod python;

pub use javascript::*;
pub use python::*;
