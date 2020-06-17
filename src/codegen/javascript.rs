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

pub struct JavaScript;

fn is_reserved(name: &str) -> bool {
    RESERVED_WORDS.contains(&name)
}

fn is_numeric(name: &str) -> bool {
    if let Some(c) = name.chars().next() {
        c.is_numeric()
    } else {
        false
    }
}

fn is_underscore(name: &str) -> bool {
    if let Some(c) = name.chars().next() {
        c == '_'
    } else {
        false
    }
}


impl CodegenTarget for JavaScript {
    fn generate_identifier<'i>(&self, ident: &Identifier<'i>) -> String {
        if is_reserved(ident.0) || is_numeric(ident.0) || is_underscore(ident.0) {
            format!("_{}", ident.0)
        } else {
            ident.0.to_string()
        }
    }

    fn generate_lambda<'i>(&self, lambda: &Lambda<'i>) -> String {
        format!("{} => {}", self.generate_identifier(&lambda.0), self.generate_application(&lambda.1))
    }

    fn generate_expression<'i>(&self, expr: &Expression<'i>) -> String {
        match expr {
            Expression::Identifier(ident) => self.generate_identifier(ident),
            Expression::Parenthesis(app) => format!("({})", self.generate_application(app)),
            Expression::Lambda(lambda) => self.generate_lambda(lambda)
        }
    }

    fn generate_application<'i>(&self, app: &Application<'i>) -> String {
        let mut iter = app.0.iter();
        let mut res = String::new();

        if let Some(expr) = iter.next() {
            res += &self.generate_expression(expr);
        }

        for expr in iter {
            res += &format!("({})", self.generate_expression(expr));
        }

        res
    }

    fn generate_assignment<'i>(&self, ass: &Assignment<'i>) -> String {
        format!("const {} = {};", self.generate_identifier(&ass.0), self.generate_application(&ass.1))
    }

    fn generate_program<'i>(&self, program: &Program<'i>) -> String {
        let mut res = String::new();

        for ass in program.0.iter() {
            res += &format!("{}\n", self.generate_assignment(ass));
        }

        res
    }
}
