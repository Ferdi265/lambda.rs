use super::data::*;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

impl<D: ASTData> Display for Lambda<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;
        write!(f, "{} -> {}", self.argument, self.body)
    }
}

impl<D: ASTData> Display for Expression<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expression::Lambda(lambda) => write!(f, "{}", lambda),
            Expression::Parenthesis(app) => write!(f, "({})", app),
            Expression::Identifier(ident) => write!(f, "{}", ident)
        }
    }
}

impl<D: ASTData> Display for Application<'_, D> {
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

impl<D: ASTData> Display for Assignment<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;
        write!(f, "{} = {}", self.target, self.value)
    }
}

impl<D: ASTData> Display for Program<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.data.fmt(f)?;

        for ass in self.assignments.iter() {
            writeln!(f, "{}", ass)?;
        }

        Ok(())
    }
}

impl<D: ASTData> Debug for Lambda<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<D: ASTData> Debug for Expression<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<D: ASTData> Debug for Application<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<D: ASTData> Debug for Assignment<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}

impl<D: ASTData> Debug for Program<'_, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST({})", self)
    }
}
