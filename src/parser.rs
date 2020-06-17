use pest::Parser;
use pest::iterators::Pair as PestPair;
use pest::iterators::Pairs as PestPairs;
use pest::error::Error as PestError;
use pest_derive::Parser;

use crate::error::Error;
use crate::ast::*;

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
    fn test_parenthesis() {
        assert_eq!(
            LambdaParser::parse_parenthesis("((a))"),
            Ok(Application(vec![
                Expression::Parenthesis(Application(vec![
                    Expression::Identifier(Identifier("a"))
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_parenthesis("(a (b c) ((d) e))"),
            Ok(Application(vec![
                Expression::Identifier(Identifier("a")),
                Expression::Parenthesis(Application(vec![
                    Expression::Identifier(Identifier("b")),
                    Expression::Identifier(Identifier("c")),
                ])),
                Expression::Parenthesis(Application(vec![
                    Expression::Parenthesis(Application(vec![
                        Expression::Identifier(Identifier("d"))
                    ])),
                    Expression::Identifier(Identifier("e"))
                ]))
            ]))
        );
    }

    #[test]
    fn test_expression() {
        assert_eq!(
            LambdaParser::parse_expression("e -> (a -> a) (c -> c) e"),
            Ok(Expression::Lambda(Lambda(Identifier("e"), Application(vec![
                Expression::Parenthesis(Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Identifier(Identifier("a"))
                    ])))
                ])),
                Expression::Parenthesis(Application(vec![
                    Expression::Lambda(Lambda(Identifier("c"), Application(vec![
                        Expression::Identifier(Identifier("c"))
                    ])))
                ])),
                Expression::Identifier(Identifier("e"))
            ]))))
        );
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

    #[test]
    fn test_assignment() {
        assert_eq!(
            LambdaParser::parse_assignment("ident = a -> a"),
            Ok(Assignment(Identifier("ident"), Application(vec![
                Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                    Expression::Identifier(Identifier("a"))
                ])))
            ])))
        );
        assert_eq!(
            LambdaParser::parse_assignment("and = a -> b -> a b false"),
            Ok(Assignment(Identifier("and"), Application(vec![
                Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("b")),
                        Expression::Identifier(Identifier("false"))
                    ])))
                ])))
            ])))
        );
    }

    #[test]
    fn test_program() {
        assert_eq!(
            LambdaParser::parse_program("true = a -> b -> a"),
            Ok(Program(vec![
                Assignment(Identifier("true"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("a"))
                        ])))
                    ])))
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_program(r"
                true = a -> b -> a
            "),
            Ok(Program(vec![
                Assignment(Identifier("true"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("a"))
                        ])))
                    ])))
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_program("true = a -> b -> a\nfalse = a -> b -> b"),
            Ok(Program(vec![
                Assignment(Identifier("true"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("a"))
                        ])))
                    ])))
                ])),
                Assignment(Identifier("false"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("b"))
                        ])))
                    ])))
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_program(r"
                true = a -> b -> a
                false = a -> b -> b
                not = a -> a false true
            "),
            Ok(Program(vec![
                Assignment(Identifier("true"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("a"))
                        ])))
                    ])))
                ])),
                Assignment(Identifier("false"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("b"))
                        ])))
                    ])))
                ])),
                Assignment(Identifier("not"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("false")),
                        Expression::Identifier(Identifier("true"))
                    ])))
                ]))
            ]))
        );
    }

    #[test]
    fn test_program_newlines() {
        assert_eq!(
            LambdaParser::parse_program(r"
                true =
                    a ->
                    b ->
                    a
                false
                    = a
                    ->
                    b -> b
                not = a -> a false true
            "),
            Ok(Program(vec![
                Assignment(Identifier("true"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("a"))
                        ])))
                    ])))
                ])),
                Assignment(Identifier("false"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Lambda(Lambda(Identifier("b"), Application(vec![
                            Expression::Identifier(Identifier("b"))
                        ])))
                    ])))
                ])),
                Assignment(Identifier("not"), Application(vec![
                    Expression::Lambda(Lambda(Identifier("a"), Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("false")),
                        Expression::Identifier(Identifier("true"))
                    ])))
                ]))
            ]))
        );
        assert_eq!(
            LambdaParser::parse_program(r"
                x =
                    (a b)
                y = (
                    a b)
                z = (a b
                    )
            "),
            Ok(Program(vec![
                Assignment(Identifier("x"), Application(vec![
                    Expression::Parenthesis(Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("b"))
                    ]))
                ])),
                Assignment(Identifier("y"), Application(vec![
                    Expression::Parenthesis(Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("b"))
                    ]))
                ])),
                Assignment(Identifier("z"), Application(vec![
                    Expression::Parenthesis(Application(vec![
                        Expression::Identifier(Identifier("a")),
                        Expression::Identifier(Identifier("b"))
                    ]))
                ]))
            ]))
        );
    }
}
