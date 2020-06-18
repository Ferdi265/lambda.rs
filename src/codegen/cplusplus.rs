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

static IMPLEMENTATION_WORDS: [&str; 3] = [
    "lambda",
    "captures",
    "arg"
];

static CODEGEN_PRELUDE: &str = include_str!("prelude.cpp");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CPlusPlus;

struct Context<'i> {
    current_assignment: Identifier<'i>,
    current_impls: Vec<String>
}

impl<'i> Context<'i> {
    fn new(current_assignment: Identifier<'i>) -> Self {
        Context {
            current_assignment,
            current_impls: Vec::new()
        }
    }
}

fn is_reserved(name: &str) -> bool {
    RESERVED_WORDS.contains(&name) || IMPLEMENTATION_WORDS.contains(&name)
}

fn is_numeric(name: &str) -> bool {
    name.starts_with(char::is_numeric)
}

fn is_underscore(name: &str) -> bool {
    name.starts_with('_')
}

fn is_end_underscore(name: &str) -> bool {
    name.ends_with('_')
}

impl CPlusPlus {
    fn generate_internal_identifier(&self, ident: Identifier<'_>) -> String {
        let mut gen = ident.to_string();

        if is_reserved(ident) || is_numeric(ident) || is_underscore(ident) {
            gen.insert(0, '_');
        }

        gen
    }

    fn generate_identifier(&self, ident: Identifier<'_>) -> String {
        let mut gen = self.generate_internal_identifier(ident);

        if is_end_underscore(ident) {
            gen.push('_');
        }

        gen
    }

    fn generate_lambda_identifier(&self, ident: Identifier<'_>, id: usize) -> String {
        self.generate_internal_identifier(ident) + &format!("{}_", id)
    }

    fn generate_lambda_impl(&self, ctx: &mut Context<'_>, lambda: &Lambda<'_>) {
        let ident = self.generate_lambda_identifier(ctx.current_assignment, lambda.id);

        let mut res = format!("lambda {}(lambda captures[], lamda arg) {{\n", ident);

        for (i, cap) in lambda.captures.iter().enumerate() {
            let cap_ident = self.generate_identifier(cap);
            res += &format!("    lambda {} = captures[{}]\n", cap_ident, i);
        }

        let arg_ident = self.generate_identifier(lambda.argument);
        res += &format!("    lambda {} = arg\n", arg_ident);

        res += "    ";
        res += &self.generate_application(ctx, &lambda.body);
        res += ";\n}\n";

        ctx.current_impls.push(res);
    }

    fn generate_lambda_instance(&self, ctx: &mut Context<'_>, lambda: &Lambda<'_>) -> String {
        let ident = self.generate_lambda_identifier(ctx.current_assignment, lambda.id);

        let mut res = format!("lambda({}, {{", ident);

        for (i, cap) in lambda.captures.iter().enumerate() {
            if i != 0 {
                res += ", ";
            }

            res += &self.generate_identifier(cap);
        }

        res + "})"
    }

    fn generate_lambda(&self, ctx: &mut Context<'_>, lambda: &Lambda<'_>) -> String {
        self.generate_lambda_impl(ctx, lambda);
        self.generate_lambda_instance(ctx, lambda)
    }

    fn generate_expression(&self, ctx: &mut Context<'_>, expr: &Expression<'_>) -> String {
        match expr {
            Expression::Identifier(ident) => self.generate_identifier(ident),
            Expression::Parenthesis(app) => format!("({})", self.generate_application(ctx, app)),
            Expression::Lambda(lambda) => self.generate_lambda(ctx, lambda)
        }
    }

    fn generate_application(&self, ctx: &mut Context<'_>, app: &Application<'_>) -> String {
        let mut iter = app.expressions.iter();
        let mut res = String::new();

        if let Some(expr) = iter.next() {
            res += &self.generate_expression(ctx, expr);
        }

        for expr in iter {
            res += &format!("({})", self.generate_expression(ctx, expr));
        }

        res
    }

    fn generate_assignment(&self, ass: &Assignment<'_>) -> String {
        let mut ctx = Context::new(ass.target);

        let target = self.generate_identifier(ass.target);
        let app = self.generate_application(&mut ctx, &ass.value);

        let mut res = String::new();

        for imp in ctx.current_impls {
            res += &imp;
        }

        res + &format!("lambda {} = {};", target, app)
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
