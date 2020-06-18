use super::*;

#[test]
fn test_identifier() {
    assert_eq!(LambdaParser::parse_identifier("true"), Ok("true"));
    assert_eq!(LambdaParser::parse_identifier("1st"), Ok("1st"));
    assert_eq!(LambdaParser::parse_identifier("2nd"), Ok("2nd"));

    assert!(LambdaParser::parse_identifier("+").is_err());
    assert!(LambdaParser::parse_identifier("(").is_err());
    assert!(LambdaParser::parse_identifier(")").is_err());
}

#[test]
fn test_lambda() {
    assert_eq!(
        LambdaParser::parse_lambda("a -> b"),
        Ok(Lambda { argument: "a", body: Application { expressions: vec![
            Expression::Identifier("b")
        ]}})
    );
    assert_eq!(
        LambdaParser::parse_lambda("a -> b -> c"),
        Ok(Lambda { argument: "a", body: Application { expressions: vec![
            Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                Expression::Identifier("c")
            ]}})
        ]}})
    );
    assert_eq!(
        LambdaParser::parse_lambda("a -> b c"),
        Ok(Lambda { argument: "a", body: Application { expressions: vec![
            Expression::Identifier("b"),
            Expression::Identifier("c")
        ]}})
    );

    assert!(LambdaParser::parse_lambda("(a -> b) -> c").is_err());
}

#[test]
fn test_parenthesis() {
    assert_eq!(
        LambdaParser::parse_parenthesis("((a))"),
        Ok(Application { expressions: vec![
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Identifier("a")
            ]})
        ]})
    );
    assert_eq!(
        LambdaParser::parse_parenthesis("(a (b c) ((d) e))"),
        Ok(Application { expressions: vec![
            Expression::Identifier("a"),
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Identifier("b"),
                Expression::Identifier("c"),
            ]}),
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Identifier("d")
                ]}),
                Expression::Identifier("e")
            ]})
        ]})
    );
}

#[test]
fn test_expression() {
    assert_eq!(
        LambdaParser::parse_expression("e -> (a -> a) (c -> c) e"),
        Ok(Expression::Lambda(Lambda { argument: "e", body: Application { expressions: vec![
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Identifier("a")
                ]}})
            ]}),
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "c", body: Application { expressions: vec![
                    Expression::Identifier("c")
                ]}})
            ]}),
            Expression::Identifier("e")
        ]}}))
    );
}

#[test]
fn test_application() {
    assert_eq!(
        LambdaParser::parse_application("a b"),
        Ok(Application { expressions: vec![
            Expression::Identifier("a"),
            Expression::Identifier("b")
        ]})
    );
    assert_eq!(
        LambdaParser::parse_application("a b c"),
        Ok(Application { expressions: vec![
            Expression::Identifier("a"),
            Expression::Identifier("b"),
            Expression::Identifier("c")
        ]})
    );
    assert_eq!(
        LambdaParser::parse_application("(a b) c"),
        Ok(Application { expressions: vec![
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Identifier("a"),
                Expression::Identifier("b"),
            ]}),
            Expression::Identifier("c")
        ]})
    );
    assert_eq!(
        LambdaParser::parse_application("a (b c)"),
        Ok(Application { expressions: vec![
            Expression::Identifier("a"),
            Expression::Parenthesis(Application { expressions: vec![
                Expression::Identifier("b"),
                Expression::Identifier("c"),
            ]})
        ]})
    );
    assert_eq!(
        LambdaParser::parse_application("a b -> c"),
        Ok(Application { expressions: vec![
            Expression::Identifier("a"),
            Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                Expression::Identifier("c")
            ]}})
        ]})
    );
}

#[test]
fn test_assignment() {
    assert_eq!(
        LambdaParser::parse_assignment("ident = a -> a"),
        Ok(Assignment { target: "ident", value: Application { expressions: vec![
            Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                Expression::Identifier("a")
            ]}})
        ]}})
    );
    assert_eq!(
        LambdaParser::parse_assignment("and = a -> b -> a b false"),
        Ok(Assignment { target: "and", value: Application { expressions: vec![
            Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("b"),
                    Expression::Identifier("false")
                ]}})
            ]}})
        ]}})
    );
}

#[test]
fn test_program() {
    assert_eq!(
        LambdaParser::parse_program("true = a -> b -> a"),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program(r"
            true = a -> b -> a
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program("true = a -> b -> a\nfalse = a -> b -> b"),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "false", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("b")
                    ]}})
                ]}})
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program(r"
            true = a -> b -> a
            false = a -> b -> b
            not = a -> a false true
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "false", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("b")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "not", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("false"),
                    Expression::Identifier("true")
                ]}})
            ]}}
        ]})
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
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "false", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("b")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "not", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("false"),
                    Expression::Identifier("true")
                ]}})
            ]}}
        ]})
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
        Ok(Program { assignments: vec![
            Assignment { target: "x", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("b")
                ]})
            ]}},
            Assignment { target: "y", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("b")
                ]})
            ]}},
            Assignment { target: "z", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("b")
                ]})
            ]}}
        ]})
    );
}

#[test]
fn test_program_inner_newlines() {
    assert_eq!(
        LambdaParser::parse_program(r"
            x = (
                a
                b
                )
            y = a
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "x", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("b")
                ]})
            ]}},
            Assignment { target: "y", value: Application { expressions: vec![
                Expression::Identifier("a"),
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program(r"
            x = (a -> b
                c
            )
            y = a
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "x", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                        Expression::Identifier("b"),
                        Expression::Identifier("c")
                    ]}}),
                ]})
            ]}},
            Assignment { target: "y", value: Application { expressions: vec![
                Expression::Identifier("a"),
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program(r"
            x = ((a -> b)
                c
            )
            y = a
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "x", value: Application { expressions: vec![
                Expression::Parenthesis(Application { expressions: vec![
                    Expression::Parenthesis(Application { expressions: vec![
                        Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                            Expression::Identifier("b"),
                        ]}}),
                    ]}),
                    Expression::Identifier("c")
                ]})
            ]}},
            Assignment { target: "y", value: Application { expressions: vec![
                Expression::Identifier("a"),
            ]}}
        ]})
    );

    assert!(
        LambdaParser::parse_program(r"
            x =
                a
                b
            y = a
        ").is_err()
    );
    assert!(
        LambdaParser::parse_program(r"
            x = a -> b
                c
            y = a
        ").is_err()
    );
    assert!(
        LambdaParser::parse_program(r"
            x = (a -> b)
                c
            y = a
        ").is_err()
    );
}

#[test]
fn test_program_comments() {
    assert_eq!(
        LambdaParser::parse_program(r"
            # the two possible bools
            true = a -> b -> a
            false = a -> b -> b

            # unary bool combinators
            not = a -> a false true
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "false", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("b")
                    ]}})
                ]}})
            ]}},
            Assignment { target: "not", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Identifier("a"),
                    Expression::Identifier("false"),
                    Expression::Identifier("true")
                ]}})
            ]}}
        ]})
    );
    assert_eq!(
        LambdaParser::parse_program(r"# foo
            # bar

            # the two possible bools
            true = a -> b -> a
        "),
        Ok(Program { assignments: vec![
            Assignment { target: "true", value: Application { expressions: vec![
                Expression::Lambda(Lambda { argument: "a", body: Application { expressions: vec![
                    Expression::Lambda(Lambda { argument: "b", body: Application { expressions: vec![
                        Expression::Identifier("a")
                    ]}})
                ]}})
            ]}}
        ]})
    );
}
