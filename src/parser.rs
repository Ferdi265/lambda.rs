use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;
