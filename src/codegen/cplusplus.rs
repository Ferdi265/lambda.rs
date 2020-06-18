use super::*;

static RESERVED_WORDS: [&str; 95] = [
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
];

static IMPLEMENTATION_WORDS: [&str; 1] = [
    "lambda"
];

static CODEGEN_PRELUDE: &str = include_str!("prelude.cpp");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CPlusPlus;

fn is_reserved(name: &str) -> bool {
    RESERVED_WORDS.contains(&name) || IMPLEMENTATION_WORDS.contains(&name)
}

fn is_numeric(name: &str) -> bool {
    name.starts_with(char::is_numeric)
}

fn is_underscore(name: &str) -> bool {
    name.starts_with('_')
}

impl CPlusPlus {
    fn generate_identifier(&self, ident: Identifier<'_>) -> String {
        let mut gen = ident.to_string();

        if is_reserved(ident) || is_numeric(ident) || is_underscore(ident) {
            gen.insert(0, '_');
        }

        gen
    }

    fn generate_lambda(&self, lambda: &Lambda<'_>) -> String {
        let ident = self.generate_identifier(lambda.argument);
        let body = self.generate_application(&lambda.body);

        format!("lambda([=](lambda {}) {{ return {}; }})", ident, body)
    }

    fn generate_expression(&self, expr: &Expression<'_>) -> String {
        match expr {
            Expression::Identifier(ident) => self.generate_identifier(ident),
            Expression::Parenthesis(app) => format!("({})", self.generate_application(app)),
            Expression::Lambda(lambda) => self.generate_lambda(lambda)
        }
    }

    fn generate_application(&self, app: &Application<'_>) -> String {
        let mut iter = app.expressions.iter();
        let mut res = String::new();

        if let Some(expr) = iter.next() {
            res += &self.generate_expression(expr);
        }

        for expr in iter {
            res += &format!("({})", self.generate_expression(expr));
        }

        res
    }

    fn generate_assignment(&self, ass: &Assignment<'_>) -> String {
        let target = self.generate_identifier(ass.target);
        let app = self.generate_application(&ass.value);

        format!("lambda {} = [](){{ return {}; }}();", target, app)
    }
}

impl CodegenTarget for CPlusPlus {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        res += CODEGEN_PRELUDE;

        for ass in program.assignments.iter() {
            res += &format!("{}\n", self.generate_assignment(ass));
        }

        res
    }
}
