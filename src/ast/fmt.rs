use super::*;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Error as FmtError;

impl<'i> Display for Lambda<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} -> {}", self.argument, self.body)
    }
}

impl<'i> Display for Expression<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expression::Lambda(lambda) => write!(f, "{}", lambda),
            Expression::Parenthesis(app) => write!(f, "({})", app),
            Expression::Identifier(ident) => write!(f, "{}", ident)
        }
    }
}

impl<'i> Display for Application<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut iter = self.expressions.iter();

        if let Some(expr) = iter.next() {
            write!(f, "{}", expr)?;
        } else {
            return Err(FmtError);
        }

        for expr in iter {
            write!(f, " {}", expr)?;
        }

        Ok(())
    }
}

impl<'i> Display for Assignment<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} = {}", self.target, self.value)
    }
}

impl<'i> Display for Program<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for ass in self.assignments.iter() {
            writeln!(f, "{}", ass)?;
        }

        Ok(())
    }
}

impl<'i> Debug for Lambda<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i> Debug for Expression<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i> Debug for Application<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i> Debug for Assignment<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i> Debug for Program<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}
