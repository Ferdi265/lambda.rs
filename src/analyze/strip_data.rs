use std::rc::Rc;

use crate::ast::generic;
pub use crate::ast::nodata::*;

pub fn transform_program<'i, D: generic::ASTData<'i>>(program: &generic::Program<'i, D>) -> Program<'i> {
    Program {
        assignments: program.iter()
            .map(transform_assignment)
            .collect(),
        data: ()
    }
}

fn transform_assignment<'i, D: generic::ASTData<'i>>(ass: &generic::Assignment<'i, D>) -> Assignment<'i> {
    Assignment {
        target: ass.target,
        value: transform_application(&ass.value),
        data: ()
    }
}

fn transform_application<'i, D: generic::ASTData<'i>>(app: &generic::Application<'i, D>) -> Rc<Application<'i>> {
    Rc::new(Application {
        head: transform_expression(&app.head),
        tail: app.tail.as_ref()
            .map(|tail| transform_application(tail)),
        data: ()
    })
}

fn transform_expression<'i, D: generic::ASTData<'i>>(expr: &generic::Expression<'i, D>) -> Expression<'i> {
    match expr {
        generic::Expression::Identifier(ident) => Expression::Identifier(ident),
        generic::Expression::Parenthesis(app) => Expression::Parenthesis(transform_application(app)),
        generic::Expression::Lambda(lambda) => Expression::Lambda(transform_lambda(lambda))
    }
}

fn transform_lambda<'i, D: generic::ASTData<'i>>(lambda: &generic::Lambda<'i, D>) -> Rc<Lambda<'i>> {
    Rc::new(Lambda {
        argument: lambda.argument,
        body: transform_application(&lambda.body),
        data: ()
    })
}
