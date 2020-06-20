use crate::ast::nodata;

pub mod compute_lambda_captures;
pub mod compute_continuations;
pub mod compute_continuation_captures;
pub mod strip_data;

pub use compute_continuation_captures::Literal;
pub use compute_continuation_captures::Continuation;
pub use compute_continuation_captures::AssignmentData;
pub use compute_continuation_captures::LambdaData;
pub use compute_continuation_captures::Identifier;
pub use compute_continuation_captures::Lambda;
pub use compute_continuation_captures::Expression;
pub use compute_continuation_captures::Application;
pub use compute_continuation_captures::Assignment;
pub use compute_continuation_captures::Program;

pub struct AnalysisResult<'i> {
    pub program: Program<'i>,
    pub diagnostics: Vec<String>
}

pub fn analyze_program<'i>(program: &nodata::Program<'i>) -> AnalysisResult<'i> {
    use compute_lambda_captures::PassResult;

    let PassResult { program, diagnostics } = compute_lambda_captures::transform_program(program);
    let program = compute_continuations::transform_program(&program);
    let program = compute_continuation_captures::transform_program(&program);

    AnalysisResult {
        program,
        diagnostics
    }
}
