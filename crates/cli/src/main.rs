use clap::Parser;
use serde_json::to_string_pretty;
use std::fs;

use runtime::exec::ExecutionRuntime;

#[derive(Parser, Debug)]
#[command(name = "apisql")]
#[command(about = "Run .apisql queries against JSON APIs")]
struct Args {
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source = fs::read_to_string(&args.file)?;
    let mut executor = ExecutionRuntime::new();
    let result = executor.run_source(&source)?;

    println!("{}", to_string_pretty(&result)?);
    Ok(())
}
