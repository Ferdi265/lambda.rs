use crate::parser::ParseError;
use crate::ast::maker::AstMakeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ParseError(ParseError),
    AstMakeError(AstMakeError)
}
