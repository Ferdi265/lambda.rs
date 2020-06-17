use crate::ast::*;

pub trait CodegenTarget {
    fn generate_identifier<'i>(&self, ident: &Identifier<'i>) -> String;
    fn generate_lambda<'i>(&self, lambda: &Lambda<'i>) -> String;
    fn generate_expression<'i>(&self, expr: &Expression<'i>) -> String;
    fn generate_application<'i>(&self, app: &Application<'i>) -> String;
    fn generate_assignment<'i>(&self, ass: &Assignment<'i>) -> String;
    fn generate_program<'i>(&self, program: &Program<'i>) -> String;
}

mod javascript;

pub use javascript::*;
