use crate::analyze::*;

pub trait CodegenTarget {
    fn generate(&self, program: &Program<'_>) -> String;
}

mod util;

mod cplusplus;
mod cplusplus_cps;
mod javascript;
mod lua;
mod python;

pub use cplusplus::CPlusPlus;
pub use cplusplus_cps::CPlusPlusCPS;
pub use javascript::JavaScript;
pub use lua::Lua;
pub use python::Python;
