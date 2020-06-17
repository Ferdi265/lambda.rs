use pest::Span;
use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;

pub struct Program<'i>(pub Vec<Assignment<'i>>);

pub struct Assignment<'i>(pub Identifier<'i>, Application<'i>);

pub struct Application<'i>(pub Vec<Expression<'i>>);

pub enum Expression<'i> {
    Identifier(Identifier<'i>),
    Lambda(Identifier<'i>, Application<'i>)
}

pub struct Identifier<'i>(pub Span<'i>);

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_ident() {
        let res = LambdaParser::parse(Rule::identifier, "map").unwrap();

        println!("{:?}", res);
    }
}
