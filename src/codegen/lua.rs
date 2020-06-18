use super::*;

static RESERVED_WORDS: [&str; 21] = [
    "and",
    "break",
    "do",
    "else",
    "elseif",
    "end",
    "false",
    "for",
    "function",
    "if",
    "in",
    "local",
    "nil",
    "not",
    "or",
    "repeat",
    "return",
    "then",
    "true",
    "until",
    "while"
];

#[derive(Debug, Clone, Copy)]
pub struct Lua;

fn generate_identifier(ident: Identifier<'_>) -> String {
    util::generate_identifier(ident, &RESERVED_WORDS)
}

fn generate_lambda(lambda: &Lambda<'_>) -> String {
    format!("function ({}) return {} end", generate_identifier(&lambda.argument), generate_application(&lambda.body))
}

fn generate_expression(expr: &Expression<'_>) -> String {
    match expr {
        Expression::Identifier(ident) => generate_identifier(ident),
        Expression::Parenthesis(app) => format!("({})", generate_application(app)),
        Expression::Lambda(lambda) => generate_lambda(lambda)
    }
}

fn generate_application(app: &Application<'_>) -> String {
    let mut iter = app.expressions.iter();
    let mut res = String::new();

    if let Some(expr) = iter.next() {
        res += &generate_expression(expr);
    }

    for expr in iter {
        res += &format!("({})", generate_expression(expr));
    }

    res
}

fn generate_assignment(ass: &Assignment<'_>) -> String {
    format!("{} = {}", generate_identifier(&ass.target), generate_application(&ass.value))
}

impl CodegenTarget for Lua {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        for ass in program.assignments.iter() {
            res += &format!("{}\n", generate_assignment(ass));
        }

        res
    }
}
