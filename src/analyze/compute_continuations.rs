use std::rc::Rc;
use std::collections::BTreeSet;

use crate::ast::generic;
use super::compute_lambda_captures as prev;

#[derive(Debug, Clone)]
pub enum GenericLiteral<'i, D: generic::ASTData<'i>> {
    Anonymous(usize),
    Identifier(Identifier<'i>),
    Lambda(Rc<generic::Lambda<'i, D>>)
}

#[derive(Debug, Clone)]
pub struct GenericContinuation<'i, D: generic::ASTData<'i>> {
    pub id: usize,
    pub function: GenericLiteral<'i, D>,
    pub argument: GenericLiteral<'i, D>
}

#[derive(Debug, Clone)]
pub struct GenericAssignmentData<'i, D: generic::ASTData<'i>> {
    pub continuations: Vec<GenericContinuation<'i, D>>,
    pub result_literal: Literal<'i>
}

#[derive(Debug, Clone)]
pub struct GenericLambdaData<'i, D: generic::ASTData<'i>> {
    pub id: usize,
    pub captures: BTreeSet<Identifier<'i>>,
    pub continuations: Vec<GenericContinuation<'i, D>>,
    pub result_literal: Literal<'i>
}

#[derive(Debug, Clone, Copy)]
pub struct PassData;

pub type Literal<'i> = GenericLiteral<'i, PassData>;
pub type Continuation<'i> = GenericContinuation<'i, PassData>;
pub type AssignmentData<'i> = GenericAssignmentData<'i, PassData>;
pub type LambdaData<'i> = GenericLambdaData<'i, PassData>;

impl<'i> generic::ASTData<'i> for PassData {
    type AssignmentData = AssignmentData<'i>;
    type LambdaData = LambdaData<'i>;
}

pub use generic::Identifier;
pub type Lambda<'i> = generic::Lambda<'i, PassData>;
pub type Expression<'i> = generic::Expression<'i, PassData>;
pub type Application<'i> = generic::Application<'i, PassData>;
pub type Assignment<'i> = generic::Assignment<'i, PassData>;
pub type Program<'i> = generic::Program<'i, PassData>;

#[derive(Debug, Clone)]
struct Context<'i> {
    current_id: usize,
    continuations: Vec<Continuation<'i>>
}

impl<'i> Context<'i> {
    fn new() -> Self {
        Context {
            current_id: 0,
            continuations: Vec::new()
        }
    }

    fn push(&mut self, function: Literal<'i>, argument: Literal<'i>) -> Literal<'i> {
        let id = self.current_id;
        self.current_id += 1;

        self.continuations.push(Continuation {
            id,
            function,
            argument
        });

        Literal::Anonymous(id)
    }
}

pub fn transform_program<'i>(program: &prev::Program<'i>) -> Program<'i> {
    Program {
        assignments: program.iter()
            .map(transform_assignment)
            .collect(),
        data: ()
    }
}

fn transform_assignment<'i>(ass: &prev::Assignment<'i>) -> Assignment<'i> {
    let (value, continuations, lit) = transform_application(&ass.value);

    Assignment {
        target: ass.target,
        value,
        data: AssignmentData {
            continuations,
            result_literal: lit
        }
    }
}

fn transform_application<'i>(app: &prev::Application<'i>)
    -> (Rc<Application<'i>>, Vec<Continuation<'i>>, Literal<'i>)
{
    let mut ctx = Context::new();

    let (app, lit) = transform_application_initial(app, &mut ctx, |_, lit| lit);

    (app, ctx.continuations, lit)
}

fn transform_application_initial<'i, F>(app: &prev::Application<'i>, ctx: &mut Context<'i>, f: F)
    -> (Rc<Application<'i>>, Literal<'i>)
    where F: FnOnce(&mut Context<'i>, Literal<'i>) -> Literal<'i>
{
    let next = app.tail
        .as_ref()
        .map(|app| &**app);

    let (head, lit) = transform_expression(&app.head, ctx);
    let lit = f(ctx, lit);
    let (tail, lit) = transform_application_continuation(next, ctx, lit);

    (Rc::new(Application {
        head,
        tail,
        data: ()
    }), lit)
}

fn transform_application_continuation<'i>(app: Option<&prev::Application<'i>>, ctx: &mut Context<'i>, lit1: Literal<'i>)
    -> (Option<Rc<Application<'i>>>, Literal<'i>)
{
    if let Some(app) = app {
        let (app, lit2) = transform_application_initial(app, ctx, move |ctx, lit2| ctx.push(lit1, lit2));

        (Some(app), lit2)
    } else {
        (None, lit1)
    }
}

fn transform_expression<'i>(expr: &prev::Expression<'i>, ctx: &mut Context<'i>)
    -> (Expression<'i>, Literal<'i>)
{
    match expr {
        prev::Expression::Parenthesis(nonlit) => {
            let (head_app, anon1) = transform_application_initial(&nonlit, ctx, |_, lit| lit);

            (Expression::Parenthesis(head_app), anon1)
        },
        prev::Expression::Lambda(lambda) => {
            let lambda = transform_lambda(&lambda);

            (Expression::Lambda(lambda.clone()), Literal::Lambda(lambda))
        }
        prev::Expression::Identifier(ident) => {
            (Expression::Identifier(ident), Literal::Identifier(ident))
        }
    }
}

fn transform_lambda<'i>(lambda: &prev::Lambda<'i>) -> Rc<Lambda<'i>> {
    let (body, continuations, lit) = transform_application(&lambda.body);

    Rc::new(Lambda {
        argument: lambda.argument,
        body,
        data: LambdaData {
            id: lambda.data.id,
            captures: lambda.data.captures.clone(),
            continuations,
            result_literal: lit
        }
    })
}
