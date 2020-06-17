use std::str::FromStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::fs::read_to_string;

use structopt::StructOpt;

use lambda::error::Error;
use lambda::parser::LambdaParser;
use lambda::codegen::*;

#[derive(StructOpt)]
#[structopt(about = "a simple functional language inspired by the lambda calculus")]
enum Options {
    Pretty {
        #[structopt(parse(from_os_str))]
        file: PathBuf
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
        Options::Pretty { file } => file,
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

    match opt {
        Options::Pretty { .. } => print!("{}", parsed),
        Options::Codegen { target, .. } => print!("{}", target.generate_program(&parsed))
    }

    Ok(())
}
