use crate::parser::ParseError;
use crate::ast::AstError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ParseError(ParseError),
    AstError(AstError)
}
