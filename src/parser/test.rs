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
