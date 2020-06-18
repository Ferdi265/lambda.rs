use super::*;

static RESERVED_WORDS: [&str; 64] = [
    "abstract",
    "arguments",
    "await",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "double",
    "else",
    "enum",
    "eval",
    "export",
    "extends",
    "false",
    "final",
    "finally",
    "float",
    "for",
    "function",
    "goto",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "int",
    "interface",
    "let",
    "long",
    "native",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "volatile",
    "while",
    "with",
    "yield"
];

#[derive(Debug, Clone, Copy)]
pub struct JavaScript;

fn generate_identifier(ident: Identifier<'_>) -> String {
    util::generate_identifier(ident, &RESERVED_WORDS)
}

fn generate_lambda(lambda: &Lambda<'_>) -> String {
    format!("{} => {}", generate_identifier(&lambda.argument), generate_application(&lambda.body))
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
    format!("const {} = {};", generate_identifier(&ass.target), generate_application(&ass.value))
}

impl CodegenTarget for JavaScript {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        for ass in program.assignments.iter() {
            res += &format!("{}\n", generate_assignment(ass));
        }

        res
    }
}
