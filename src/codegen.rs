use crate::analyze::*;

pub trait CodegenTarget {
    fn generate(&self, program: &Program<'_>) -> String;
}

mod util;

mod cplusplus;
mod javascript;
mod lua;
mod python;

pub use cplusplus::CPlusPlus;
pub use javascript::JavaScript;
pub use lua::Lua;
pub use python::Python;
