use crate::ast::*;

pub trait CodegenTarget {
    fn generate_identifier<'i>(ident: &Identifier<'i>) -> String;
    fn generate_lambda<'i>(lambda: &Lambda<'i>) -> String;
    fn generate_expression<'i>(expr: &Expression<'i>) -> String;
    fn generate_application<'i>(app: &Application<'i>) -> String;
    fn generate_assignment<'i>(ass: &Assignment<'i>) -> String;
    fn generate_program<'i>(program: &Program<'i>) -> String;
}

mod javascript;

pub use javascript::*;
