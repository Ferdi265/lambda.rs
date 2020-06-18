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

impl JavaScript {
    fn generate_identifier(&self, ident: Identifier<'_>) -> String {
        if is_reserved(ident) || is_numeric(ident) || is_underscore(ident) {
            format!("_{}", ident)
        } else {
            ident.to_string()
        }
    }

    fn generate_lambda(&self, lambda: &Lambda<'_>) -> String {
        format!("{} => {}", self.generate_identifier(&lambda.argument), self.generate_application(&lambda.body))
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
        format!("const {} = {};", self.generate_identifier(&ass.target), self.generate_application(&ass.value))
    }

}

impl CodegenTarget for JavaScript {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        for ass in program.assignments.iter() {
            res += &format!("{}\n", self.generate_assignment(ass));
        }

        res
    }
}
