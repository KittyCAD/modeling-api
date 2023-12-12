//! Establish a modeling session with the KittyCAD API.

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use kittycad::{types::error::Error as ApiError, Client};
use kittycad_modeling_cmds::{
    ok_response::OkModelingCmdResponse,
    websocket::{
        FailureWebSocketResponse, ModelingCmdReq, OkWebSocketResponseData, WebSocketRequest, WebSocketResponse,
    },
    ModelingCmd,
};
use reqwest::Upgraded;
use tokio_tungstenite::{tungstenite::Message as WsMsg, WebSocketStream};
use uuid::Uuid;

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
}

/// An active session with the KittyCAD Modeling API.
/// TODO: This needs some sort of buffering. It should allow users to send many requests in a row and then wait for the responses.
pub struct Session {
    write_to_ws: SplitSink<WebSocketStream<Upgraded>, WsMsg>,
    read_from_ws: SplitStream<WebSocketStream<Upgraded>>,
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
        }: SessionBuilder,
    ) -> Result<Self, ApiError> {
        // TODO: establish WebRTC connections for the user.
        let webrtc = Some(false);
        let ws = client
            .modeling()
            .commands_ws(fps, unlocked_framerate, video_res_height, video_res_width, webrtc)
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
        Ok(Self {
            write_to_ws,
            read_from_ws,
        })
    }

    /// Send a modeling command and wait for its response.
    pub async fn run_command<'de, Cmd>(&mut self, cmd: Cmd) -> Result<OkModelingCmdResponse, RunCommandError>
    where
        Cmd: kittycad_modeling_cmds::ModelingCmdVariant<'de>,
    {
        // All messages to the KittyCAD Modeling API will be sent over the WebSocket as Text.
        // The text will contain JSON representing a `ModelingCmdReq`.
        // This takes in a command and its ID, and makes a WebSocket message containing that command.
        let cmd_id = kittycad_modeling_cmds::id::ModelingCmdId(Uuid::new_v4());
        let cmd = ModelingCmd::from(cmd);
        let ws_msg = WsMsg::Text(
            serde_json::to_string(&WebSocketRequest::ModelingCmdReq(ModelingCmdReq { cmd, cmd_id })).unwrap(),
        );
        self.write_to_ws
            .send(ws_msg)
            .await
            .map_err(RunCommandError::WebSocketSend)?;
        while let Some(msg) = self.read_from_ws.next().await {
            // We're looking for a WebSocket response with text.
            // Ignore any other type of WebSocket messages.
            let Some(resp) = text_from_ws(msg.map_err(RunCommandError::WebSocketRecv)?) else {
                continue;
            };
            // What did the WebSocket response contain?
            // It should either match the KittyCAD successful response schema, or the failed response schema.
            match decode_websocket_text(&resp, cmd_id.into())? {
                // Success!
                Ok(OkWebSocketResponseData::Modeling { modeling_response }) => {
                    return Ok(modeling_response);
                }
                // Success, but not a modeling response
                Ok(_) => {}
                // Failure
                Err(e) => {
                    return Err(RunCommandError::ModelingApiFailure {
                        request_id: e.request_id,
                        errors: e.errors,
                    })
                }
            }
        }
        Err(RunCommandError::WebSocketClosed)
    }
}

/// Given the text from a WebSocket, deserialize its JSON.
/// Returns OK if the WebSocket's JSON represents a successful response.
/// Returns an error if the WebSocket's JSON represented a failure response.
fn decode_websocket_text(
    text: &str,
    request_id: Uuid,
) -> Result<std::result::Result<OkWebSocketResponseData, FailureWebSocketResponse>, RunCommandError> {
    let resp: WebSocketResponse = serde_json::from_str(text)?;
    match resp {
        WebSocketResponse::Success(s) => {
            if s.request_id == Some(request_id) {
                assert!(s.success);
                Ok(Ok(s.resp))
            } else {
                Err(RunCommandError::WrongId)
            }
        }
        WebSocketResponse::Failure(f) => {
            assert!(!f.success);
            Ok(Err(f))
        }
    }
}

/// Find the text in a WebSocket message, if there's any.
fn text_from_ws(msg: WsMsg) -> Option<String> {
    match msg {
        WsMsg::Text(text) => Some(text),
        _ => None,
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
}
