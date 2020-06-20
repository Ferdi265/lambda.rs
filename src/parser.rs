use pest::Parser;
use pest::iterators::Pair as PestPair;
use pest::iterators::Pairs as PestPairs;
use pest::error::Error as PestError;
use pest_derive::Parser;

use crate::error::Error;
use crate::ast::nodata::*;
use crate::ast::maker::*;

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;

pub type Pair<'i> = PestPair<'i, Rule>;
pub type Pairs<'i> = PestPairs<'i, Rule>;
pub type ParseError = PestError<Rule>;

impl LambdaParser {
    fn parse_rule<'i, T, M>(code: &'i str, rule: Rule, maker: M) -> Result<T, Error>
        where T: 'i, M: Maker<'i, T>
    {
        let pairs = LambdaParser::parse(rule, code).map_err(Error::ParseError)?;

        from_pairs(pairs, maker)
    }

    pub fn parse_identifier(code: &str) -> Result<Identifier<'_>, Error> {
        Self::parse_rule(code, Rule::identifier, make_identifier)
    }

    pub fn parse_lambda(code: &str) -> Result<Lambda<'_>, Error> {
        Self::parse_rule(code, Rule::lambda, make_lambda)
    }

    pub fn parse_parenthesis(code: &str) -> Result<Application<'_>, Error> {
        Self::parse_rule(code, Rule::parenthesis, make_parenthesis)
    }

    pub fn parse_expression(code: &str) -> Result<Expression<'_>, Error> {
        Self::parse_rule(code, Rule::expression, make_expression)
    }

    pub fn parse_application(code: &str) -> Result<Application<'_>, Error> {
        Self::parse_rule(code, Rule::application, make_application)
    }

    pub fn parse_assignment(code: &str) -> Result<Assignment<'_>, Error> {
        Self::parse_rule(code, Rule::assignment, make_assignment)
    }

    pub fn parse_program(code: &str) -> Result<Program<'_>, Error> {
        Self::parse_rule(code, Rule::program, make_program)
    }
}

#[cfg(test)]
mod test;
