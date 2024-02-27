//! Time-travelling debugger for KittyCAD execution plans.
use std::{env, io, process::exit};

use anyhow::{bail, Context, Result};
use kittycad_execution_plan::Instruction;
use kittycad_modeling_session::Session;

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
    let session = client().await?;
    let (history, last_instruction) =
        kittycad_execution_plan::execute_time_travel(&mut mem, plan.clone(), Some(session)).await;
    app::run(app::Context {
        last_instruction,
        history,
        plan,
    })
}

async fn client() -> Result<Session> {
    let Ok(kittycad_api_token) = env::var("KITTYCAD_API_TOKEN")else {
        anyhow::bail!("You must set $KITTYCAD_API_TOKEN")
    };
    let kittycad_api_client = kittycad::Client::new(kittycad_api_token);
    let session_builder = kittycad_modeling_session::SessionBuilder {
        client: kittycad_api_client,
        fps: Some(10),
        unlocked_framerate: Some(false),
        video_res_height: Some(720),
        video_res_width: Some(1280),
        buffer_reqs: None,
        await_response_timeout: None,
    };
    match Session::start(session_builder).await {
        Err(e) => {
            return Err(match e {
                kittycad::types::error::Error::InvalidRequest(s) => {
                    anyhow::anyhow!("Request did not meet requirements {s}")
                }
                kittycad::types::error::Error::CommunicationError(e) => {
                    anyhow::anyhow!(" A server error either due to the data, or with the connection: {e}")
                }
                kittycad::types::error::Error::RequestError(e) => anyhow::anyhow!("Could not build request: {e}"),
                kittycad::types::error::Error::SerdeError { error, status } => {
                    anyhow::anyhow!("Serde error (HTTP {status}): {error}")
                }
                kittycad::types::error::Error::InvalidResponsePayload { error, response } => {
                    anyhow::anyhow!("Invalid response payload. Error {error}, response {response:?}")
                }
                kittycad::types::error::Error::Server { body, status } => {
                    anyhow::anyhow!("Server error (HTTP {status}): {body}")
                }
                kittycad::types::error::Error::UnexpectedResponse(resp) => {
                    let status = resp.status();
                    let url = resp.url().to_owned();
                    match resp.text().await {
                        Ok(body) => anyhow::anyhow!(
                            "Unexpected response from KittyCAD API.
                        URL:{url}
                        HTTP {status}
                        ---Body----
                        {body}"
                        ),
                        Err(e) => anyhow::anyhow!(
                            "Unexpected response from KittyCAD API.
                        URL:{url}
                        HTTP {status}
                        ---Body could not be read, the error is----
                        {e}"
                        ),
                    }
                }
            })
        }
        Ok(x) => Ok(x),
    }
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
