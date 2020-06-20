use super::data::*;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

impl<'i, D: ASTData<'i>> Display for Lambda<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;
        write!(f, "{} -> {}", self.argument, self.body)
    }
}

impl<'i, D: ASTData<'i>> Display for Expression<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expression::Lambda(lambda) => write!(f, "{}", lambda),
            Expression::Parenthesis(app) => write!(f, "({})", app),
            Expression::Identifier(ident) => write!(f, "{}", ident)
        }
    }
}

impl<'i, D: ASTData<'i>> Display for Application<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;
        write!(f, "{}", self.head)?;

        if let Some(tail) = &self.tail {
            write!(f, " {}", tail)
        } else {
            Ok(())
        }
    }
}

impl<'i, D: ASTData<'i>> Display for Assignment<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;
        write!(f, "{} = {}", self.target, self.value)
    }
}

impl<'i, D: ASTData<'i>> Display for Program<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;

        for ass in self.assignments.iter() {
            writeln!(f, "{}", ass)?;
        }

        Ok(())
    }
}

impl<'i, D: ASTData<'i>> Debug for Lambda<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i, D: ASTData<'i>> Debug for Expression<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i, D: ASTData<'i>> Debug for Application<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i, D: ASTData<'i>> Debug for Assignment<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<'i, D: ASTData<'i>> Debug for Program<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}
