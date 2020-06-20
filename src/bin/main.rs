use std::str::FromStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::fs::read_to_string;

use structopt::StructOpt;

use lambda::error::Error;
use lambda::parser::LambdaParser;
use lambda::analyze::analyze_program;
use lambda::analyze::strip_data;
use lambda::codegen::*;

#[derive(StructOpt)]
#[structopt(about = "a simple functional language inspired by the lambda calculus")]
#[structopt(rename_all = "kebab-case")]
enum Options {
    Check {
        #[structopt(parse(from_os_str))]
        file: PathBuf
    },
    Pretty {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Codegen {
        #[structopt(parse(from_os_str))]
        file: PathBuf,

        #[structopt(long)]
        target: Target
    }
}

struct Target(Box<dyn CodegenTarget>);

impl FromStr for Target {
    type Err = &'static str;
    fn from_str(target: &str) -> Result<Target, Self::Err> {
        Ok(Target(match target {
            "c++" | "cplusplus" | "cxx" | "cpp" => Box::new(CPlusPlus),
            "javascript" | "js" => Box::new(JavaScript),
            "lua" => Box::new(Lua),
            "python" | "py" => Box::new(Python),
            _ => return Err("unsupported target")
        }))
    }
}

impl Deref for Target {
    type Target = dyn CodegenTarget;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

fn main() -> Result<(), String> {
    let opt = Options::from_args();

    let file = match &opt {
        Options::Check { file, .. } => file,
        Options::Pretty { file, .. } => file,
        Options::Codegen { file, .. } => file
    };

    let code = read_to_string(file)
        .map_err(|e| format!("failed to read file: {}", e))?;

    let parsed = match LambdaParser::parse_program(&code) {
        Ok(program) => program,
        Err(Error::ParseError(e)) => {
            eprintln!("\n{}", e);
            return Err(String::from("failed to parse program"));
        }
        Err(Error::AstMakeError(_)) => return Err(String::from("failed to build AST"))
    };

    let analyze_result = analyze_program(&parsed);
    for diagnostic in analyze_result.diagnostics {
        println!("{}", diagnostic);
    }

    let stripped = strip_data::transform_program(&analyze_result.program);

    match opt {
        Options::Check { .. } => {}
        Options::Pretty { .. } => print!("{}", stripped),
        Options::Codegen { target, .. } => print!("{}", target.generate(&analyze_result.program))
    }

    Ok(())
}
