//! Instruction for running KittyCAD API requests.

use std::collections::HashMap;

use crate::events::{Event, Severity};
use crate::Result;
use crate::{events::EventWriter, memory::Memory};
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
    ) -> Result<()> {
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
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::MovePathPen => {
                let cmd = each_cmd::MovePathPen::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::ExtendPath => {
                let cmd = each_cmd::ExtendPath::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::ClosePath => {
                let cmd = each_cmd::ClosePath::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::Extrude => {
                let cmd = each_cmd::Extrude::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::TakeSnapshot => {
                let cmd = each_cmd::TakeSnapshot::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::MakePlane => {
                let cmd = each_cmd::MakePlane::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::EnableSketchMode => {
                let cmd = each_cmd::EnableSketchMode::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::SketchModeEnable => {
                let cmd = each_cmd::SketchModeEnable::from_memory(&mut arguments, mem, events)?;
                log_req(events);
                session.run_command(cmd_id, cmd).await?
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
}

pub(crate) async fn execute_batch(
    reqs: Vec<ApiRequest>,
    session: &mut ModelingSession,
    mem: &mut Memory,
    events: &mut EventWriter,
) -> Result<()> {
    // Batch the requests.
    let reqs_by_id: HashMap<_, _> = reqs.iter().map(|req| (req.cmd_id, req.clone())).collect();
    let batch = reqs
        .iter()
        .map(|req| {
            let cmd_id = req.cmd_id;
            let mut arguments = req.arguments.clone().into_iter();
            let cmd = match &req.endpoint {
                Endpoint::StartPath => {
                    let cmd = each_cmd::StartPath::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::MovePathPen => {
                    let cmd = each_cmd::MovePathPen::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::ExtendPath => {
                    let cmd = each_cmd::ExtendPath::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::ClosePath => {
                    let cmd = each_cmd::ClosePath::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::Extrude => {
                    let cmd = each_cmd::Extrude::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::TakeSnapshot => {
                    let cmd = each_cmd::TakeSnapshot::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::MakePlane => {
                    let cmd = each_cmd::MakePlane::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::EnableSketchMode => {
                    let cmd = each_cmd::EnableSketchMode::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                Endpoint::SketchModeEnable => {
                    let cmd = each_cmd::SketchModeEnable::from_memory(&mut arguments, mem, events)?;
                    ModelingCmd::from(cmd)
                }
                other => panic!("Haven't implemented endpoint {other:?} yet"),
            };
            Ok(ModelingCmdReq { cmd, cmd_id })
        })
        .collect::<Result<Vec<_>>>()
        .map(|requests| ModelingBatch { requests })?;
    let resps = session.run_batch(batch).await?;
    for resp in resps {
        let store_response = reqs_by_id.get(&resp.cmd_id).unwrap().store_response;
        if let Some(output_address) = store_response {
            events.push(Event {
                text: "Storing response".to_owned(),
                severity: Severity::Info,
                related_addresses: vec![output_address],
            });
            mem.set_composite(output_address, resp.response);
        }
    }
    Ok(())
}
