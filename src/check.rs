use std::collections::BTreeSet;

use crate::ast::*;
use crate::ast::checked;

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
pub struct CheckResult<'i> {
    pub program: checked::Program<'i>,
    pub diagnostics: Vec<String>
}

pub fn check_program<'i>(program: &Program<'i>) -> CheckResult<'i> {
    let mut ctx = Context::new("");

    let asss: Vec<_> = program.iter()
        .map(|ass| {
            let ass = check_assignment(ass, &mut ctx);
            ctx.add_global(ass.target);
            ass
        })
        .collect();

    CheckResult {
        program: checked::Program { assignments: asss },
        diagnostics: ctx.diagnostics
    }
}

fn check_assignment<'i>(ass: &Assignment<'i>, ctx: &mut Context<'i>) -> checked::Assignment<'i> {
    if ctx.contains(ass.target) {
        ctx.add_diagnostic("error", format!("redefinition of '{}'", ass.target));
    }

    ctx.set_assignment(ass.target);

    checked::Assignment {
        target: ass.target,
        value: check_application(&ass.value, ctx)
    }
}

fn check_application<'i>(app: &Application<'i>, ctx: &mut Context<'i>) -> checked::Application<'i> {
    let exprs: Vec<_> = app.iter()
        .map(|expr| check_expression(expr, ctx))
        .collect();

    checked::Application {
        expressions: exprs
    }
}

fn check_expression<'i>(expr: &Expression<'i>, ctx: &mut Context<'i>) -> checked::Expression<'i> {
    match expr {
        Expression::Identifier(ident) => {
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

            checked::Expression::Identifier(ident)
        },
        Expression::Parenthesis(app) => checked::Expression::Parenthesis(check_application(app, ctx)),
        Expression::Lambda(lambda) => checked::Expression::Lambda(check_lambda(lambda, ctx))
    }
}

fn check_lambda<'i>(lambda: &Lambda<'i>, ctx: &mut Context<'i>) -> checked::Lambda<'i> {
    let id = ctx.get_id();
    let mut subctx = ctx.split_with_local(lambda.argument);

    let body = check_application(&lambda.body, &mut subctx);

    subctx.referenced.remove(lambda.argument);
    let captures: BTreeSet<_> = subctx.referenced.intersection(&ctx.locals)
        .copied().collect();

    ctx.merge(subctx);

    checked::Lambda {
        id,
        argument: lambda.argument,
        body,
        captures
    }
}
