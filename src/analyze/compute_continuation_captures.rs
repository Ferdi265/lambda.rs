use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::ast::generic;
use super::compute_continuations as prev;

use prev::GenericLiteral;

#[derive(Debug, Clone)]
pub struct GenericContinuation<'i, D: generic::ASTData<'i>> {
    pub id: usize,
    pub function: GenericLiteral<'i, D>,
    pub argument: GenericLiteral<'i, D>,
    pub captures: BTreeSet<Identifier<'i>>,
    pub anonymous_captures: BTreeSet<usize>
}

#[derive(Debug, Clone)]
pub struct GenericAssignmentData<'i, D: generic::ASTData<'i>> {
    pub continuations: Vec<GenericContinuation<'i, D>>,
}

#[derive(Debug, Clone)]
pub struct GenericLambdaData<'i, D: generic::ASTData<'i>> {
    pub id: usize,
    pub captures: BTreeSet<Identifier<'i>>,
    pub continuations: Vec<GenericContinuation<'i, D>>,
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
    lambdas: BTreeMap<usize, Rc<Lambda<'i>>>
}

impl<'i> Context<'i> {
    fn new() -> Self {
        Context {
            lambdas: BTreeMap::new()
        }
    }

    fn push(&mut self, lambda: Rc<Lambda<'i>>) {
        self.lambdas.insert(lambda.data.id, lambda);
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
    let (value, continuations) = transform_continuations(&ass.value, &ass.data.continuations);

    Assignment {
        target: ass.target,
        value,
        data: AssignmentData {
            continuations
        }
    }
}

fn transform_continuations<'i>(app: &prev::Application<'i>, continuations: &[prev::Continuation<'i>])
    -> (Rc<Application<'i>>, Vec<Continuation<'i>>)
{
    let mut ctx = Context::new();
    let app = transform_application(app, &mut ctx);

    let mut continuations: Vec<_> = continuations.iter()
        .map(|cont| transform_continuation(cont, &mut ctx))
        .collect();

    let mut next: Option<&Continuation<'i>> = None;
    for cur in continuations.iter_mut().rev() {
        if let Some(next) = next {
            cur.captures.extend(&next.captures);
            cur.anonymous_captures.extend(&next.anonymous_captures);
            cur.anonymous_captures.remove(&cur.id);
        }

        if let Literal::Lambda(lambda) = &cur.function {
            cur.captures.extend(&lambda.data.captures)
        }

        if let Literal::Lambda(lambda) = &cur.argument {
            cur.captures.extend(&lambda.data.captures)
        }

        next = Some(cur);
    }

    (app, continuations)
}

fn transform_continuation<'i>(cont: &prev::Continuation<'i>, ctx: &mut Context<'i>) -> Continuation<'i> {
    Continuation {
        id: cont.id,
        function: tranform_literal(&cont.function, ctx),
        argument: tranform_literal(&cont.argument, ctx),
        captures: BTreeSet::new(),
        anonymous_captures: BTreeSet::new()
    }
}

fn tranform_literal<'i>(lit: &prev::Literal<'i>, ctx: &mut Context<'i>) -> Literal<'i> {
    match lit {
        prev::Literal::Anonymous(id) => Literal::Anonymous(*id),
        prev::Literal::Identifier(ident) => Literal::Identifier(ident),
        prev::Literal::Lambda(lambda) => Literal::Lambda(
            ctx.lambdas.remove(&lambda.data.id).expect("did not find lambda in AST")
        )
    }
}

fn transform_application<'i>(app: &prev::Application<'i>, ctx: &mut Context<'i>) -> Rc<Application<'i>> {
    Rc::new(Application {
        head: transform_expression(&app.head, ctx),
        tail: app.tail.as_ref()
            .map(|app| transform_application(app, ctx)),
        data: ()
    })
}

fn transform_expression<'i>(expr: &prev::Expression<'i>, ctx: &mut Context<'i>) -> Expression<'i> {
    match expr {
        prev::Expression::Parenthesis(app) => Expression::Parenthesis(transform_application(app, ctx)),
        prev::Expression::Lambda(lambda) => Expression::Lambda(transform_lambda(lambda, ctx)),
        prev::Expression::Identifier(ident) => Expression::Identifier(ident)
    }
}

fn transform_lambda<'i>(lambda: &prev::Lambda<'i>, ctx: &mut Context<'i>) -> Rc<Lambda<'i>> {
    let (body, continuations) = transform_continuations(&lambda.body, &lambda.data.continuations);

    let lambda = Rc::new(Lambda {
        argument: lambda.argument,
        body,
        data: LambdaData {
            id: lambda.data.id,
            captures: lambda.data.captures.clone(),
            continuations
        }
    });

    ctx.push(lambda.clone());

    lambda
}
