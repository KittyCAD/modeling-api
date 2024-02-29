//! Instruction for executing file-system commands.
//! Initially based off of ApiRequest.

use crate::events::{Event, Severity};
use crate::Result;
use crate::{events::EventWriter, memory::Memory};
use kittycad_execution_plan_traits::{Address, FromMemory, InMemory};
use kittycad_modeling_cmds::{each_cmd, id::ModelingCmdId, ModelingCmdEndpoint as Endpoint};
use serde::{Deserialize, Serialize};

/// A command to give to the file-system.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FsCommand {
  /// Check if a file exists.
  Exists,
}

/// A request to the file-system.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FsRequest {
    /// Which ModelingCmd to call.
    pub command: FsCommand,
    /// Which address should the response be stored in?
    /// If none, the response will be ignored.
    pub store_response: Option<Address>,
    /// Look up each parameter at this address.
    pub arguments: Vec<InMemory>,
}

impl FsRequest {
    /// Execute this file-system request.
    pub async fn execute(
        self,
        mem: &mut Memory,
        events: &mut EventWriter,
    ) -> Result<()> {
        let Self {
            command,
            store_response,
            arguments,
        } = self;
        let mut arguments = arguments.into_iter();
        events.push(Event {
            text: "Reading parameters".to_owned(),
            severity: Severity::Debug,
            related_address: Default::default(),
        });
        let log_req = || {
            events.push(Event {
                text: "Parameters read".to_owned(),
                severity: Severity::Debug,
                related_address: Default::default(),
            });
            events.push(Event {
                text: "Executing command".to_owned(),
                severity: Severity::Info,
                related_address: Default::default(),
            });
        };
        let output = match command {
            FsCommand::Exists => {
                // let cmd = each_cmd::StartPath::from_memory(&mut arguments, mem)?;
                // log_req();
                // session.run_command(cmd_id, cmd).await?
                true
            }
        };
        // Write out to memory.
        if let Some(output_address) = store_response {
            events.push(Event {
                text: "Storing response".to_owned(),
                severity: Severity::Info,
                related_address: Some(output_address),
            });
            mem.set_composite(output_address, output);
        }
        Ok(())
    }
}
