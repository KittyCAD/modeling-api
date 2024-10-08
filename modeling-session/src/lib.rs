//! Establish a modeling session with the KittyCAD API.

use std::time::Duration;

use futures::StreamExt;
use kittycad::{types::error::Error as ApiError, Client};
use kittycad_modeling_cmds::{
    id::ModelingCmdId,
    ok_response::OkModelingCmdResponse,
    websocket::{ModelingBatch, ModelingCmdReq},
    ModelingCmd,
};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

mod actor;

/// Parameters for starting a session with the KittyCAD Modeling API.
pub struct SessionBuilder {
    /// Client to the KittyCAD API.
    pub client: Client,
    ///  Frames per second of the video feed.
    pub fps: Option<u32>,
    ///  If true, engine will render video frames as fast as it can.
    pub unlocked_framerate: Option<bool>,
    /// Height of the video feed. Must be a multiple of 4.
    pub video_res_height: Option<u32>,
    /// Width of the video feed. Must be a multiple of 4.
    pub video_res_width: Option<u32>,
    /// How many requests for sending/receiving to/from the API can be in-flight at once.
    pub buffer_reqs: Option<usize>,
    /// How long to wait for the response to a modeling command.
    /// Defaults to 10 seconds.
    pub await_response_timeout: Option<Duration>,
    /// Show the grid?
    pub show_grid: Option<bool>,
}

/// An active session with the KittyCAD Modeling API.
/// TODO: This needs some sort of buffering. It should allow users to send many requests in a row and then wait for the responses.
pub struct Session {
    actor_tx: mpsc::Sender<actor::Request>,
}

impl Session {
    /// Start a session.
    pub async fn start(
        SessionBuilder {
            client,
            fps,
            unlocked_framerate,
            video_res_height,
            video_res_width,
            buffer_reqs,
            await_response_timeout,
            show_grid,
        }: SessionBuilder,
    ) -> Result<Self, ApiError> {
        // TODO: establish WebRTC connections for the user.
        let webrtc = Some(false);
        let (ws, _headers) = client
            .modeling()
            .commands_ws(
                fps,
                None,
                None,
                None,
                show_grid,
                unlocked_framerate,
                video_res_height,
                video_res_width,
                webrtc,
            )
            .await?;
        // Now that we have a WebSocket connection, we can split it into two ends:
        // one for writing to and one for reading from.
        let (write_to_ws, read_from_ws) = tokio_tungstenite::WebSocketStream::from_raw_socket(
            ws,
            tokio_tungstenite::tungstenite::protocol::Role::Client,
            None,
        )
        .await
        .split();
        let (actor_tx, actor_rx) = mpsc::channel(buffer_reqs.unwrap_or(10));
        tokio::task::spawn(actor::start(
            actor_rx,
            write_to_ws,
            read_from_ws,
            await_response_timeout.unwrap_or(Duration::from_secs(10)),
        ));
        Ok(Self { actor_tx })
    }

    /// Send a modeling command and wait for its response.
    pub async fn run_command(
        &mut self,
        cmd_id: ModelingCmdId,
        cmd: ModelingCmd,
    ) -> Result<OkModelingCmdResponse, RunCommandError> {
        // All messages to the KittyCAD Modeling API will be sent over the WebSocket as Text.
        // The text will contain JSON representing a `ModelingCmdReq`.
        // This takes in a command and its ID, and makes a WebSocket message containing that command.
        let (tx, rx) = oneshot::channel();
        self.actor_tx
            .send(actor::Request::SendModelingCmd(ModelingCmdReq { cmd, cmd_id }, tx))
            .await
            .map_err(|_| RunCommandError::ActorFailed)?;
        rx.await.map_err(|_| RunCommandError::ActorFailed)??;
        let (tx, rx) = oneshot::channel();
        self.actor_tx
            .send(actor::Request::GetResponse(cmd_id, tx))
            .await
            .map_err(|_| RunCommandError::ActorFailed)?;
        let resp = rx.await.map_err(|_| RunCommandError::ActorFailed)??;
        Ok(resp)
    }

    /// Run a batch of commands at once.
    pub async fn run_batch_no_responses(
        &mut self,
        requests: Vec<ModelingCmdReq>,
        batch_id: ModelingCmdId,
    ) -> Result<(), RunCommandError> {
        let (tx, rx) = oneshot::channel();
        self.actor_tx
            .send(actor::Request::SendModelingBatch(
                ModelingBatch {
                    requests,
                    batch_id,
                    responses: false,
                },
                tx,
            ))
            .await
            .map_err(|_| RunCommandError::ActorFailed)?;
        rx.await.map_err(|_| RunCommandError::ActorFailed)??;
        Ok(())
    }
}

/// Errors from running a modeling command.
#[derive(thiserror::Error, Debug)]
pub enum RunCommandError {
    /// Error from the KittyCAD API client.
    #[error("error from KittyCAD API client: {0}")]
    ApiError(#[from] ApiError),
    /// Request body could not be serialized.
    #[error("the given body couldn't be serialized: {0}")]
    InvalidRequestBody(#[from] serde_json::Error),
    /// Could not send message via WebSocket.
    #[error("could not send via WebSocket: {0}")]
    WebSocketSend(tokio_tungstenite::tungstenite::Error),
    /// Could not receive message via WebSocket.
    #[error("could not receive via WebSocket: {0}")]
    WebSocketRecv(tokio_tungstenite::tungstenite::Error),
    /// Modeling API request failed.
    #[error("modeling API returned an error on request {request_id:?}: {errors:?}")]
    ModelingApiFailure {
        /// ID of the failed request.
        request_id: Option<Uuid>,
        /// Errors that caused the request to fail.
        errors: Vec<kittycad_modeling_cmds::websocket::ApiError>,
    },
    /// WebSocket closed unexpectedly.
    #[error("WebSocket closed unexpectedly")]
    WebSocketClosed,
    /// Received a response for an unexpected request ID.
    #[error("Received a response for an unexpected request ID")]
    WrongId,
    /// Timed out waiting for a response.
    #[error("Timed out waiting for a response")]
    TimeOutWaitingForResponse,
    /// Server returned the wrong type.
    #[error("Server returned the wrong type")]
    ServerSentWrongType,
    /// Actor has failed
    #[error("Websocket actor has failed, restart the session")]
    ActorFailed,
}

impl RunCommandError {
    /// Does this error indicate that the session has become unhealthy and should be restarted
    /// (i.e. ended and started again)?
    pub fn should_end_session(&self) -> bool {
        match self {
            RunCommandError::WebSocketClosed => true,
            RunCommandError::ActorFailed => true,
            RunCommandError::ApiError(_) => false,
            RunCommandError::InvalidRequestBody(_) => false,
            RunCommandError::WebSocketSend(_) => false,
            RunCommandError::WebSocketRecv(_) => false,
            RunCommandError::ModelingApiFailure { .. } => false,
            RunCommandError::WrongId => false,
            RunCommandError::TimeOutWaitingForResponse => false,
            RunCommandError::ServerSentWrongType => false,
        }
    }
}
