use super::*;

static RESERVED_WORDS: [&str; 96] = [
    "alignas",
    "alignof",
    "and",
    "and_eq",
    "asm",
    "atomic_cancel",
    "atomic_commit",
    "atomic_noexcept",
    "auto",
    "bitand",
    "bitor",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char8_t",
    "char16_t",
    "char32_t",
    "class",
    "compl",
    "concept",
    "const",
    "consteval",
    "constexpr",
    "constinit",
    "const_cast",
    "continue",
    "co_await",
    "co_return",
    "co_yield",
    "decltypedefault",
    "delete",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "not",
    "not_eq",
    "nullptr",
    "operator",
    "or",
    "or_eq",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "requires",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "synchronized",
    "template",
    "this",
    "thread_local",
    "throw",
    "true",
    "try",
    "typedef",
    "typeid",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
    "xor",
    "xor_eq ",
    "lambda"
];

static CODEGEN_PRELUDE: &str = include_str!("prelude.cpp");

#[derive(Debug, Clone, Copy)]
pub struct CPlusPlus;

fn generate_identifier(ident: Identifier<'_>) -> String {
    util::generate_identifier(ident, &RESERVED_WORDS)
}

fn generate_lambda(lambda: &Lambda<'_>) -> String {
    let ident = generate_identifier(lambda.argument);
    let body = generate_application(&lambda.body);

    format!("lambda([=](lambda {}) {{ return {}; }})", ident, body)
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
    let target = generate_identifier(ass.target);
    let app = generate_application(&ass.value);

    format!("lambda {} = [](){{ return {}; }}();", target, app)
}

impl CodegenTarget for CPlusPlus {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        res += CODEGEN_PRELUDE;

        for ass in program.assignments.iter() {
            res += &format!("{}\n", generate_assignment(ass));
        }

        res
    }
}
