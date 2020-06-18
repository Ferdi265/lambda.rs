use std::collections::HashSet;

use crate::ast::*;
use crate::ast::checked;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Context<'i> {
    current_assignment: Identifier<'i>,
    diagnostics: Vec<String>,
    globals: HashSet<Identifier<'i>>,
    locals: HashSet<Identifier<'i>>,
    referenced: HashSet<Identifier<'i>>
}

impl<'i> Context<'i> {
    fn new(current_assignment: Identifier<'i>) -> Self {
        Context {
            current_assignment,
            diagnostics: Vec::new(),
            globals: HashSet::new(),
            locals: HashSet::new(),
            referenced: HashSet::new()
        }
    }

    fn merge(&mut self, other: Self) {
        self.diagnostics.extend(other.diagnostics);
        self.referenced.extend(other.referenced);
    }

    fn split(&self) -> Self {
        Context {
            current_assignment: self.current_assignment,
            diagnostics: Vec::new(),
            globals: self.globals.clone(),
            locals: self.locals.clone(),
            referenced: HashSet::new()
        }
    }

    fn split_with_local(&self, local: Identifier<'i>) -> Self {
        let mut ctx = self.split();
        ctx.add_local(local);
        ctx
    }

    fn add_diagnostic(&mut self, level: &str, msg: String) {
        self.diagnostics.push(format!(
            "{}: in definition of '{}': {}",
            level, self.current_assignment, msg
        ));
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

    let asss: Vec<_> = program.assignments.iter()
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
    ctx.current_assignment= ass.target;

    checked::Assignment {
        target: ass.target,
        value: check_application(&ass.value, ctx)
    }
}

fn check_application<'i>(app: &Application<'i>, ctx: &mut Context<'i>) -> checked::Application<'i> {
    let exprs: Vec<_> = app.expressions.iter()
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
                ctx.add_diagnostic("error", format!("undefined name '{}'", ident));
            }

            checked::Expression::Identifier(ident)
        },
        Expression::Parenthesis(app) => checked::Expression::Parenthesis(check_application(app, ctx)),
        Expression::Lambda(lambda) => checked::Expression::Lambda(check_lambda(lambda, ctx))
    }
}

fn check_lambda<'i>(lambda: &Lambda<'i>, ctx: &mut Context<'i>) -> checked::Lambda<'i> {
    let mut subctx = ctx.split_with_local(lambda.argument);

    let body = check_application(&lambda.body, &mut subctx);

    subctx.referenced.remove(lambda.argument);
    let captures: HashSet<_> = subctx.referenced.intersection(&ctx.locals)
        .copied().collect();

    ctx.merge(subctx);

    checked::Lambda {
        argument: lambda.argument,
        body,
        captures
    }
}
