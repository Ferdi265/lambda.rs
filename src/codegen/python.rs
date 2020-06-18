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

pub struct Python;

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

impl CodegenTarget for Python {
    fn generate_identifier<'i>(&self, ident: Identifier<'i>) -> String {
        if is_reserved(ident) || is_numeric(ident) || is_underscore(ident) {
            format!("_{}", ident)
        } else {
            ident.to_string()
        }
    }

    fn generate_lambda<'i>(&self, lambda: &Lambda<'i>) -> String {
        format!("lambda {}: {}", self.generate_identifier(&lambda.argument), self.generate_application(&lambda.body))
    }

    fn generate_expression<'i>(&self, expr: &Expression<'i>) -> String {
        match expr {
            Expression::Identifier(ident) => self.generate_identifier(ident),
            Expression::Parenthesis(app) => format!("({})", self.generate_application(app)),
            Expression::Lambda(lambda) => self.generate_lambda(lambda)
        }
    }

    fn generate_application<'i>(&self, app: &Application<'i>) -> String {
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

    fn generate_assignment<'i>(&self, ass: &Assignment<'i>) -> String {
        format!("{} = {}", self.generate_identifier(&ass.target), self.generate_application(&ass.value))
    }

    fn generate_program<'i>(&self, program: &Program<'i>) -> String {
        let mut res = String::new();

        for ass in program.assignments.iter() {
            res += &format!("{}\n", self.generate_assignment(ass));
        }

        res
    }
}