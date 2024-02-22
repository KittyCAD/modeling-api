//! Time-travelling debugger for KittyCAD execution plans.
use std::{env, io, process::exit};

use anyhow::{bail, Context, Result};
use kittycad_execution_plan::Instruction;

mod app;
mod ui;

const INVALID_JSON: &str =
    "Invalid JSON, the JSON you supplied was invalid or does not match the Execution Plan instruction schema";

#[tokio::main]
async fn main() {
    if let Err(e) = inner_main().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn inner_main() -> Result<()> {
    let plan = get_instrs()?;
    let mut mem = kittycad_execution_plan::Memory::default();
    let session = None;
    let (history, last_instruction) =
        kittycad_execution_plan::execute_time_travel(&mut mem, plan.clone(), session).await;
    app::run(app::Context {
        last_instruction,
        history,
        plan,
    })
}

fn get_instrs() -> Result<Vec<Instruction>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    // Which file did the user tell us to read from?
    let filepath = &args[1];

    // Read the JSON.
    let serialized = match filepath.as_ref() {
        "-" => {
            let mut buf = Vec::new();
            io::copy(&mut io::stdin(), &mut buf)?;
            String::from_utf8(buf)?
        }
        "" => {
            bail!("First argument to this program must be a filepath or '-' for stdin.")
        }
        path => {
            std::fs::read_to_string(path).context(format!("could not read file from the supplied filepath '{path}'"))?
        }
    };

    // Deserialize the JSON.
    serde_json::from_str(&serialized).context(INVALID_JSON)
}

fn print_help() {
    println!("Usage: ep-debugger [FILE]");
    println!();
    println!("Accepts a list of Execution Plan instructions as JSON array.");
    println!();
    println!("Examples:");
    println!("  `ep-debugger -` reads from stdin.");
    println!("  `ep-debugger path/to/file` reads from the given filepath.");
}
