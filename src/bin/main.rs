use std::str::FromStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::fs::read_to_string;

use structopt::StructOpt;

use lambda::error::Error;
use lambda::parser::LambdaParser;
use lambda::check::check_program;
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
        match target {
            "javascript" | "js" => Ok(Target(Box::new(JavaScript))),
            "python" | "py" => Ok(Target(Box::new(Python))),
            "c++" | "cplusplus" | "cxx" | "cpp" => Ok(Target(Box::new(CPlusPlus))),
            _ => Err("unsupported target")
        }
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
        Err(Error::AstError(_)) => return Err(String::from("failed to build AST"))
    };

    let check_result = check_program(&parsed);
    for diagnostic in check_result.diagnostics {
        println!("{}", diagnostic);
    }

    match opt {
        Options::Check { .. } => {}
        Options::Pretty { .. } => print!("{}", parsed),
        Options::Codegen { target, .. } => print!("{}", target.generate(&check_result.program))
    }

    Ok(())
}
