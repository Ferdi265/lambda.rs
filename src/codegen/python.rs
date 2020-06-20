use super::*;

static RESERVED_WORDS: [&str; 34] = [
    "and",
    "as",
    "assert",
    "async",
    "break",
    "class",
    "continue",
    "def",
    "del",
    "elif",
    "else",
    "except",
    "finally",
    "False",
    "for",
    "from",
    "global",
    "if",
    "import",
    "in",
    "is",
    "lambda",
    "None",
    "nonlocal",
    "not",
    "or",
    "pass",
    "raise",
    "return",
    "True",
    "try",
    "while",
    "with",
    "yield"
];

#[derive(Debug, Clone, Copy)]
pub struct Python;

fn generate_identifier(ident: Identifier<'_>) -> String {
    util::generate_identifier(ident, &RESERVED_WORDS)
}

fn generate_lambda(lambda: &Lambda<'_>) -> String {
    format!("lambda {}: {}", generate_identifier(&lambda.argument), generate_application(&lambda.body))
}

fn generate_expression(expr: &Expression<'_>) -> String {
    match expr {
        Expression::Identifier(ident) => generate_identifier(ident),
        Expression::Parenthesis(app) => format!("({})", generate_application(app)),
        Expression::Lambda(lambda) => generate_lambda(lambda)
    }
}

fn generate_application(app: &Application<'_>) -> String {
    let mut iter = app.iter();
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

impl CodegenTarget for Python {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        for ass in program.iter() {
            res += &format!("{}\n", generate_assignment(ass));
        }

        res
    }
}
