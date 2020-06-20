use std::rc::Rc;
use std::collections::BTreeSet;

use crate::ast::generic;
use crate::ast::nodata as prev;

#[derive(Debug, Clone)]
pub struct LambdaData<'i> {
    pub id: usize,
    pub captures: BTreeSet<Identifier<'i>>
}

#[derive(Debug, Clone, Copy)]
pub struct PassData;

impl<'i> generic::ASTData<'i> for PassData {
    type LambdaData = LambdaData<'i>;
}

pub use generic::Identifier;
pub type Lambda<'i> = generic::Lambda<'i, PassData>;
pub type Expression<'i> = generic::Expression<'i, PassData>;
pub type Application<'i> = generic::Application<'i, PassData>;
pub type Assignment<'i> = generic::Assignment<'i, PassData>;
pub type Program<'i> = generic::Program<'i, PassData>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Context<'i> {
    current_assignment: Identifier<'i>,
    current_id: usize,
    diagnostics: Vec<String>,
    globals: BTreeSet<Identifier<'i>>,
    locals: BTreeSet<Identifier<'i>>,
    referenced: BTreeSet<Identifier<'i>>
}

impl<'i> Context<'i> {
    fn new(current_assignment: Identifier<'i>) -> Self {
        Context {
            current_assignment,
            current_id: 0,
            diagnostics: Vec::new(),
            globals: BTreeSet::new(),
            locals: BTreeSet::new(),
            referenced: BTreeSet::new()
        }
    }

    fn merge(&mut self, other: Self) {
        self.diagnostics.extend(other.diagnostics);
        self.referenced.extend(other.referenced);
        self.current_id = other.current_id;
    }

    fn split(&self) -> Self {
        Context {
            current_assignment: self.current_assignment,
            current_id: self.current_id,
            diagnostics: Vec::new(),
            globals: self.globals.clone(),
            locals: self.locals.clone(),
            referenced: BTreeSet::new()
        }
    }

    fn split_with_local(&self, local: Identifier<'i>) -> Self {
        let mut ctx = self.split();
        ctx.add_local(local);
        ctx
    }

    fn get_id(&mut self) -> usize {
        let id = self.current_id;
        self.current_id += 1;

        id
    }

    fn set_assignment(&mut self, ident: Identifier<'i>) {
        self.current_assignment = ident;
        self.current_id = 0;
    }

    fn add_diagnostic(&mut self, level: &str, msg: String) {
        self.diagnostics.push(format!("{}: {}", level, msg));
    }

    fn add_global(&mut self, ident: Identifier<'i>) {
        self.globals.insert(ident);
    }

    fn add_local(&mut self, ident: Identifier<'i>) {
        self.locals.insert(ident);
    }

    fn add_referenced(&mut self, ident: Identifier<'i>) {
        self.referenced.insert(ident);
    }

    fn contains(&self, ident: Identifier<'i>) -> bool {
        self.locals.contains(ident) || self.globals.contains(ident)
    }
}

#[derive(Debug, Clone)]
pub struct PassResult<'i> {
    pub program: Program<'i>,
    pub diagnostics: Vec<String>
}

pub fn transform_program<'i>(program: &prev::Program<'i>) -> PassResult<'i> {
    let mut ctx = Context::new("");

    let asss: Vec<_> = program.iter()
        .map(|ass| {
            let ass = transform_assignment(ass, &mut ctx);
            ctx.add_global(ass.target);
            ass
        })
        .collect();

    PassResult {
        program: Program { assignments: asss, data: () },
        diagnostics: ctx.diagnostics
    }
}

fn transform_assignment<'i>(ass: &prev::Assignment<'i>, ctx: &mut Context<'i>) -> Assignment<'i> {
    if ctx.contains(ass.target) {
        ctx.add_diagnostic("error", format!("redefinition of '{}'", ass.target));
    }

    ctx.set_assignment(ass.target);

    Assignment {
        target: ass.target,
        value: transform_application(&ass.value, ctx),
        data: ()
    }
}

fn transform_application<'i>(app: &prev::Application<'i>, ctx: &mut Context<'i>) -> Rc<Application<'i>> {
    let head = transform_expression(&app.head, ctx);
    let tail = app.tail.as_ref()
        .map(|tail| transform_application(&tail, ctx));

    Rc::new(Application {
        head,
        tail,
        data: ()
    })
}

fn transform_expression<'i>(expr: &prev::Expression<'i>, ctx: &mut Context<'i>) -> Expression<'i> {
    match expr {
        prev::Expression::Identifier(ident) => {
            ctx.add_referenced(ident);

            if !ctx.contains(ident) {
                if ident == &ctx.current_assignment {
                    ctx.add_diagnostic("error", format!(
                        "name '{}' referenced in its definition",
                        ident
                    ));
                } else {
                    ctx.add_diagnostic("error", format!(
                        "undefined name '{}' in definition of '{}'",
                        ctx.current_assignment, ident
                    ));
                }
            }

            Expression::Identifier(ident)
        },
        prev::Expression::Parenthesis(app) => Expression::Parenthesis(transform_application(app, ctx)),
        prev::Expression::Lambda(lambda) => Expression::Lambda(transform_lambda(lambda, ctx))
    }
}

fn transform_lambda<'i>(lambda: &prev::Lambda<'i>, ctx: &mut Context<'i>) -> Rc<Lambda<'i>> {
    let id = ctx.get_id();
    let mut subctx = ctx.split_with_local(lambda.argument);

    let body = transform_application(&lambda.body, &mut subctx);

    subctx.referenced.remove(lambda.argument);
    let captures: BTreeSet<_> = subctx.referenced.intersection(&ctx.locals)
        .copied().collect();

    ctx.merge(subctx);

    Rc::new(Lambda {
        argument: lambda.argument,
        body,
        data: LambdaData {
            id,
            captures
        }
    })
}
