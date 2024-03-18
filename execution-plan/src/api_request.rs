//! Instruction for running KittyCAD API requests.

use crate::events::{Event, Severity};
use crate::Result;
use crate::{events::EventWriter, memory::Memory, ExecutionError};
use kittycad_execution_plan_traits::{Address, FromMemory, InMemory};
use kittycad_modeling_cmds::websocket::{ModelingBatch, ModelingCmdReq};
use kittycad_modeling_cmds::ModelingCmd;
use kittycad_modeling_cmds::{each_cmd, id::ModelingCmdId, ModelingCmdEndpoint as Endpoint};
use kittycad_modeling_session::Session as ModelingSession;
use serde::{Deserialize, Serialize};

/// Request sent to the KittyCAD API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ApiRequest {
    /// Which ModelingCmd to call.
    pub endpoint: Endpoint,
    /// Which address should the response be stored in?
    /// If none, the response will be ignored.
    pub store_response: Option<Address>,
    /// Look up each parameter at this address.
    pub arguments: Vec<InMemory>,
    /// The ID of this command.
    pub cmd_id: ModelingCmdId,
}

impl ApiRequest {
    /// Execute this API request.
    pub async fn execute(
        self,
        session: &mut ModelingSession,
        mem: &mut Memory,
        events: &mut EventWriter,
        batch_queue: &mut ModelingBatch,
    ) -> Result<()> {
        if self.store_response.is_none() {
            return self.add_to_queue(mem, events, batch_queue).await;
        }
        if !batch_queue.is_empty() {
            flush_batch_queue(session, std::mem::take(batch_queue), events).await?;
        }
        let Self {
            endpoint,
            store_response,
            arguments,
            cmd_id,
        } = self;
        let mut arguments = arguments.into_iter();
        events.push(Event {
            text: "Reading parameters".to_owned(),
            severity: Severity::Debug,
            related_addresses: Default::default(),
        });
        let log_req = |events: &mut EventWriter| {
            events.push(Event {
                text: "Sending request".to_owned(),
                severity: Severity::Info,
                related_addresses: Default::default(),
            });
        };
        let output = match endpoint {
            Endpoint::StartPath => {
                let cmd = each_cmd::StartPath::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::MovePathPen => {
                let cmd = each_cmd::MovePathPen::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::ExtendPath => {
                let cmd = each_cmd::ExtendPath::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::ClosePath => {
                let cmd = each_cmd::ClosePath::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::Extrude => {
                let cmd = each_cmd::Extrude::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::TakeSnapshot => {
                let cmd = each_cmd::TakeSnapshot::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::ImportFiles => {
                let mut args_iter = arguments;
                let arg_opt_import_files_struct = args_iter.next();

                let Some(arg_import_files_struct_prim) = arg_opt_import_files_struct else {
                    return Err(ExecutionError::General {
                        reason: "Endpoint::ImportFiles requires an ImportFiles struct".to_string(),
                    });
                };

                let arg_import_files_struct = mem.get_in_memory::<kittycad_modeling_cmds::ImportFiles>(
                    arg_import_files_struct_prim,
                    "ImportFiles struct",
                    events,
                )?;

                log_req(events);
                let kittycad_modeling_cmds::ok_response::OkModelingCmdResponse::ImportFiles(import_files) = session
                    .run_command(cmd_id, ModelingCmd::from(arg_import_files_struct.0.clone()))
                    .await?
                else {
                    panic!("Unexpected OkModelingCmdResponse encountered");
                };

                // Transform a OkModelingCmdResponse::ImportFiles to a
                // OkModelingCmdResponse::ImportedGeometry for ease of
                // consumption by the rest of KCL.
                //
                // This is the most direct way to collect the original paths,
                // and return a structure that is, at the time of writing,
                // returned by the original KCL import() function call.
                kittycad_modeling_cmds::ok_response::OkModelingCmdResponse::ImportedGeometry(
                    kittycad_modeling_cmds::ok_response::output::ImportedGeometry {
                        id: import_files.object_id,
                        value: arg_import_files_struct.0.files.iter().fold(vec![], |mut acc, file| {
                            acc.push(file.path.clone());
                            acc
                        }),
                    },
                )
            }
            Endpoint::MakePlane => {
                let cmd = each_cmd::MakePlane::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::EnableSketchMode => {
                let cmd = each_cmd::EnableSketchMode::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::SketchModeEnable => {
                let cmd = each_cmd::SketchModeEnable::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::DefaultCameraZoom => {
                let cmd = each_cmd::DefaultCameraZoom::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            Endpoint::DefaultCameraFocusOn => {
                let cmd = each_cmd::DefaultCameraFocusOn::from_memory(&mut arguments, mem, events)?;
                println!("Got cmd from memory");
                log_req(events);
                println!("Running cmd");
                session.run_command(cmd_id, ModelingCmd::from(cmd)).await?
            }
            other => panic!("Haven't implemented endpoint {other:?} yet"),
        };
        // Write out to memory.
        if let Some(output_address) = store_response {
            events.push(Event {
                text: "Storing response".to_owned(),
                severity: Severity::Info,
                related_addresses: vec![output_address],
            });
            mem.set_composite(output_address, output);
        }
        Ok(())
    }

    async fn add_to_queue(
        self,
        mem: &mut Memory,
        events: &mut EventWriter,
        batch_queue: &mut ModelingBatch,
    ) -> Result<()> {
        let mut arguments = self.arguments.into_iter();
        let cmd: ModelingCmd = match self.endpoint {
            Endpoint::StartPath => each_cmd::StartPath::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::MovePathPen => each_cmd::MovePathPen::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::ExtendPath => each_cmd::ExtendPath::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::ClosePath => each_cmd::ClosePath::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::Extrude => each_cmd::Extrude::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::TakeSnapshot => each_cmd::TakeSnapshot::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::MakePlane => each_cmd::MakePlane::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::EnableSketchMode => each_cmd::EnableSketchMode::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::SketchModeEnable => each_cmd::SketchModeEnable::from_memory(&mut arguments, mem, events)?.into(),
            Endpoint::DefaultCameraFocusOn => {
                each_cmd::DefaultCameraFocusOn::from_memory(&mut arguments, mem, events)?.into()
            }
            other => panic!("Haven't implemented endpoint {other:?} yet"),
        };
        events.push(Event {
            text: format!("Adding {} to batch queue", self.endpoint),
            severity: Severity::Info,
            related_addresses: Default::default(),
        });
        batch_queue.push(ModelingCmdReq {
            cmd,
            cmd_id: self.cmd_id,
        });
        Ok(())
    }
}

/// Send any API requests that have been queued, in a batch.
pub async fn flush_batch_queue(
    session: &mut ModelingSession,
    batch_queue: ModelingBatch,
    events: &mut EventWriter,
) -> Result<()> {
    events.push(Event {
        text: format!("Running {} batched API calls", batch_queue.requests.len()),
        severity: Severity::Info,
        related_addresses: Default::default(),
    });
    session.run_batch(batch_queue).await?;
    Ok(())
}
