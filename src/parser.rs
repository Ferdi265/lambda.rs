use pest::Parser;
use pest::iterators::Pair as PestPair;
use pest::iterators::Pairs as PestPairs;
use pest::error::Error as PestError;
use pest_derive::Parser;

type Pair<'i> = PestPair<'i, Rule>;
type Pairs<'i> = PestPairs<'i, Rule>;
type ParseError = PestError<Rule>;

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ParseError(ParseError),
    GrammarError
}

mod make_ast {
    use super::*;

    pub trait Maker<'i, T>: FnOnce(Pair<'i>) -> Result<T, Error> {}
    impl<'i, T, U> Maker<'i, T> for U where U: FnOnce(Pair<'i>) -> Result<T, Error> {}

    pub fn grammar_error<T>() -> Result<T, Error> {
        Err(Error::GrammarError)
    }

    pub fn ensure_rule(pair: &Pair<'_>, rule: Rule) -> Result<(), Error> {
        if pair.as_rule() != rule { grammar_error()? }
        Ok(())
    }

    pub fn make_identifier(pair: Pair<'_>) -> Result<Identifier<'_>, Error> {
        ensure_rule(&pair, Rule::identifier)?;

        let ident = pair.as_str();

        let mut inner = pair.into_inner();
        if inner.next() != None { grammar_error()? }

        Ok(Identifier(ident))
    }

    pub fn make_lambda(pair: Pair<'_>) -> Result<Lambda<'_>, Error> {
        ensure_rule(&pair, Rule::lambda)?;

        let mut inner = pair.into_inner();
        let ident = inner.next().ok_or(Error::GrammarError)?;
        let expr = inner.next().ok_or(Error::GrammarError)?;
        if inner.next() != None { grammar_error()? }

        Ok(Lambda(make_identifier(ident)?, make_application(expr)?))
    }

    pub fn make_parenthesis(pair: Pair<'_>) -> Result<Application<'_>, Error> {
        ensure_rule(&pair, Rule::parenthesis)?;

        let mut inner = pair.into_inner();
        let app = inner.next().ok_or(Error::GrammarError)?;
        if inner.next() != None { grammar_error()? }

        make_application(app)
    }

    pub fn make_expression(pair: Pair<'_>) -> Result<Expression<'_>, Error> {
        ensure_rule(&pair, Rule::expression)?;

        let mut inner = pair.into_inner();
        let expr = inner.next().ok_or(Error::GrammarError)?;
        if inner.next() != None { grammar_error()? }

        match expr.as_rule() {
            Rule::lambda => make_lambda(expr).map(Expression::Lambda),
            Rule::parenthesis => make_parenthesis(expr).map(Expression::Parenthesis),
            Rule::identifier => make_identifier(expr).map(Expression::Identifier),
            _ => grammar_error()
        }
    }

    pub fn make_application(pair: Pair<'_>) -> Result<Application<'_>, Error> {
        ensure_rule(&pair, Rule::application)?;

        let exprs: Vec<_> = pair.into_inner()
            .map(make_expression)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Application(exprs))
    }

    pub fn make_assignment(pair: Pair<'_>) -> Result<Assignment<'_>, Error> {
        ensure_rule(&pair, Rule::assignment)?;

        let mut inner = pair.into_inner();
        let ident = inner.next().ok_or(Error::GrammarError)?;
        let app = inner.next().ok_or(Error::GrammarError)?;
        if inner.next() != None { grammar_error()? }

        Ok(Assignment(make_identifier(ident)?, make_application(app)?))
    }

    pub fn make_program(pair: Pair<'_>) -> Result<Program<'_>, Error> {
        ensure_rule(&pair, Rule::program)?;

        let asss: Vec<_> = pair.into_inner()
            .map(make_assignment)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program(asss))
    }

    pub fn from_pairs<'i, T, F>(mut pairs: Pairs<'i>, f: F) -> Result<T, Error>
        where T: 'i, F: Maker<'i, T>
    {
        let pair = pairs.next().ok_or(Error::GrammarError)?;
        if pairs.next() != None { grammar_error()? }

        f(pair)
    }
}

impl LambdaParser {
    fn parse_rule<'i, T, F>(code: &'i str, rule: Rule, f: F) -> Result<T, Error>
        where T: 'i, F: make_ast::Maker<'i, T>
    {
        let pairs = LambdaParser::parse(rule, code).map_err(Error::ParseError)?;

        make_ast::from_pairs(pairs, f)
    }

    pub fn parse_identifier(code: &str) -> Result<Identifier<'_>, Error> {
        Self::parse_rule(code, Rule::identifier, make_ast::make_identifier)
    }

    pub fn parse_lambda(code: &str) -> Result<Lambda<'_>, Error> {
        Self::parse_rule(code, Rule::lambda, make_ast::make_lambda)
    }

    pub fn parse_parenthesis(code: &str) -> Result<Application<'_>, Error> {
        Self::parse_rule(code, Rule::parenthesis, make_ast::make_parenthesis)
    }

    pub fn parse_expression(code: &str) -> Result<Expression<'_>, Error> {
        Self::parse_rule(code, Rule::expression, make_ast::make_expression)
    }

    pub fn parse_application(code: &str) -> Result<Application<'_>, Error> {
        Self::parse_rule(code, Rule::application, make_ast::make_application)
    }

    pub fn parse_assignment(code: &str) -> Result<Assignment<'_>, Error> {
        Self::parse_rule(code, Rule::assignment, make_ast::make_assignment)
    }

    pub fn parse_program(code: &str) -> Result<Program<'_>, Error> {
        Self::parse_rule(code, Rule::program, make_ast::make_program)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'i>(pub &'i str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda<'i>(pub Identifier<'i>, pub Application<'i>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'i> {
    Lambda(Lambda<'i>),
    Parenthesis(Application<'i>),
    Identifier(Identifier<'i>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'i>(pub Vec<Expression<'i>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<'i>(pub Identifier<'i>, Application<'i>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'i>(pub Vec<Assignment<'i>>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(LambdaParser::parse_identifier("true"), Ok(Identifier("true")));
        assert_eq!(LambdaParser::parse_identifier("1st"), Ok(Identifier("1st")));
        assert_eq!(LambdaParser::parse_identifier("2nd"), Ok(Identifier("2nd")));

        assert!(LambdaParser::parse_identifier("+").is_err());
        assert!(LambdaParser::parse_identifier("(").is_err());
        assert!(LambdaParser::parse_identifier(")").is_err());
    }

    #[test]
    fn test_lambda() {
        assert_eq!(
            LambdaParser::parse_lambda("a -> b"),
            Ok(Lambda(Identifier("a"), Application(vec![
                Expression::Identifier(Identifier("b"))
            ])))
        );
        assert_eq!(
            LambdaParser::parse_lambda("a -> b -> c"),
            Ok(Lambda(Identifier("a"), Application(vec![
                Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                    Expression::Identifier(Identifier("c"))
                ])))
            ])))
        );
        assert_eq!(
            LambdaParser::parse_lambda("a -> b c"),
            Ok(Lambda(Identifier("a"), Application(vec![
                Expression::Identifier(Identifier("b")),
                Expression::Identifier(Identifier("c"))
            ])))
        );

        assert!(LambdaParser::parse_lambda("(a -> b) -> c").is_err());
    }

    #[test]
    fn test_application() {
        assert_eq!(
            LambdaParser::parse_application("a b"),
            Ok(Application(vec![
                Expression::Identifier(Identifier("a")),
                Expression::Identifier(Identifier("b"))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_application("a b c"),
            Ok(Application(vec![
                Expression::Identifier(Identifier("a")),
                Expression::Identifier(Identifier("b")),
                Expression::Identifier(Identifier("c"))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_application("(a b) c"),
            Ok(Application(vec![
                Expression::Parenthesis(Application(vec![
                    Expression::Identifier(Identifier("a")),
                    Expression::Identifier(Identifier("b")),
                ])),
                Expression::Identifier(Identifier("c"))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_application("a (b c)"),
            Ok(Application(vec![
                Expression::Identifier(Identifier("a")),
                Expression::Parenthesis(Application(vec![
                    Expression::Identifier(Identifier("b")),
                    Expression::Identifier(Identifier("c")),
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_application("a b -> c"),
            Ok(Application(vec![
                Expression::Identifier(Identifier("a")),
                Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                    Expression::Identifier(Identifier("c"))
                ])))
            ]))
        );
    }
}
