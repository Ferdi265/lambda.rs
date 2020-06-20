use super::generic::*;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

impl<T: Debug> DataDisplay for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            write!(f, "[data = {:#?}] ", self)
        } else {
            write!(f, "[data = {:?}] ", self)
        }
    }
}

impl DataDisplay for () {
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

impl<'i, D: ASTData<'i>> Debug for Lambda<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        DataDisplay::fmt(&self.data, f)?;
        Display::fmt(self.argument, f)?;
        f.write_str(" -> ")?;
        Debug::fmt(&self.body, f)
    }
}

impl<'i, D: ASTData<'i>> Debug for Expression<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expression::Lambda(lambda) => Debug::fmt(lambda, f),
            Expression::Parenthesis(app) => {
                f.write_str("(")?;
                Debug::fmt(app, f)?;
                f.write_str(")")
            }
            Expression::Identifier(ident) => Display::fmt(ident, f)
        }
    }
}

impl<'i, D: ASTData<'i>> Debug for Application<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        DataDisplay::fmt(&self.data, f)?;
        Debug::fmt(&self.head, f)?;

        if let Some(tail) = &self.tail {
            f.write_str(" ")?;
            Debug::fmt(tail, f)
        } else {
            Ok(())
        }
    }
}

impl<'i, D: ASTData<'i>> Debug for Assignment<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        DataDisplay::fmt(&self.data, f)?;
        Display::fmt(&self.target, f)?;
        f.write_str(" = ")?;
        Debug::fmt(&self.value, f)
    }
}

impl<'i, D: ASTData<'i>> Debug for Program<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        DataDisplay::fmt(&self.data, f)?;

        for ass in self.assignments.iter() {
            Debug::fmt(ass, f)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl<'i, D: ASTData<'i>> Display for Lambda<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl<'i, D: ASTData<'i>> Display for Expression<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl<'i, D: ASTData<'i>> Display for Application<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl<'i, D: ASTData<'i>> Display for Assignment<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl<'i, D: ASTData<'i>> Display for Program<'i, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
