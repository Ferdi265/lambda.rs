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
struct PrevApplicationData<'i, 'a> {
    continuations: &'a [prev::Continuation<'i>],
    result_literal: prev::Literal<'i>,
    captures: BTreeSet<Identifier<'i>>
}

impl<'a, 'i> From<&'a prev::AssignmentData<'i>> for PrevApplicationData<'i, 'a> {
    fn from(ass: &'a prev::AssignmentData<'i>) -> Self {
        PrevApplicationData {
            continuations: &ass.continuations,
            result_literal: ass.result_literal.clone(),
            captures: BTreeSet::new()
        }
    }
}

impl<'a, 'i> From<&'a prev::LambdaData<'i>> for PrevApplicationData<'i, 'a> {
    fn from(lambda: &'a prev::LambdaData<'i>) -> Self {
        PrevApplicationData {
            continuations: &lambda.continuations,
            result_literal: lambda.result_literal.clone(),
            captures: lambda.captures.clone()
        }
    }
}

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
    let (value, continuations, lit) = transform_continuations(&ass.value, PrevApplicationData::from(&ass.data));

    Assignment {
        target: ass.target,
        value,
        data: AssignmentData {
            continuations,
            result_literal: lit
        }
    }
}

fn transform_continuations<'i>(app: &prev::Application<'i>, data: PrevApplicationData<'i, '_>)
    -> (Rc<Application<'i>>, Vec<Continuation<'i>>, Literal<'i>)
{
    let mut ctx = Context::new();
    let app = transform_application(app, &mut ctx);

    let mut continuations: Vec<_> = data.continuations.iter()
        .map(|cont| transform_continuation(cont, &mut ctx))
        .collect();

    let lit = transform_literal(&data.result_literal, &mut ctx);

    let mut next: Option<&Continuation<'i>> = None;
    for cur in continuations.iter_mut().rev() {
        if let Some(next) = next {
            cur.captures.extend(&next.captures);
            cur.anonymous_captures.extend(&next.anonymous_captures);
            cur.anonymous_captures.remove(&cur.id);
        }

        compute_literal_captures(cur.function.clone(), cur, &data.captures);
        compute_literal_captures(cur.argument.clone(), cur, &data.captures);

        next = Some(cur);
    }

    (app, continuations, lit)
}

fn transform_continuation<'i>(cont: &prev::Continuation<'i>, ctx: &mut Context<'i>) -> Continuation<'i> {
    Continuation {
        id: cont.id,
        function: transform_literal(&cont.function, ctx),
        argument: transform_literal(&cont.argument, ctx),
        captures: BTreeSet::new(),
        anonymous_captures: BTreeSet::new()
    }
}

fn transform_literal<'i>(lit: &prev::Literal<'i>, ctx: &mut Context<'i>) -> Literal<'i> {
    match lit {
        prev::Literal::Anonymous(id) => Literal::Anonymous(*id),
        prev::Literal::Identifier(ident) => Literal::Identifier(ident),
        prev::Literal::Lambda(lambda) => Literal::Lambda(
            ctx.lambdas.remove(&lambda.data.id).expect("did not find lambda in AST")
        )
    }
}

fn compute_literal_captures<'i>(lit: Literal<'i>, cont: &mut Continuation<'i>, cap: &BTreeSet<Identifier<'i>>) {
    match lit {
        Literal::Anonymous(id) => if id != cont.id - 1 {
            cont.anonymous_captures.insert(id);
        }
        Literal::Identifier(ident) => if cap.contains(ident) {
            cont.captures.insert(ident);
        }
        Literal::Lambda(lambda) => {
            cont.captures.extend(&lambda.data.captures);
        }
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
    let (body, continuations, lit) = transform_continuations(&lambda.body, PrevApplicationData::from(&lambda.data));

    let lambda = Rc::new(Lambda {
        argument: lambda.argument,
        body,
        data: LambdaData {
            id: lambda.data.id,
            captures: lambda.data.captures.clone(),
            continuations,
            result_literal: lit
        }
    });

    ctx.push(lambda.clone());

    lambda
}
