use std::{collections::HashMap, time::Duration};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use kittycad_modeling_cmds::{
    id::ModelingCmdId,
    ok_response::OkModelingCmdResponse,
    websocket::{ModelingBatch, ModelingCmdReq, OkWebSocketResponseData, WebSocketRequest, WebSocketResponse},
};
use reqwest::Upgraded;
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};
use tokio_tungstenite::{tungstenite::Message as WsMsg, WebSocketStream};

use crate::RunCommandError;

type Result<T> = std::result::Result<T, RunCommandError>;

pub enum Request {
    SendModelingCmd(ModelingCmdReq, oneshot::Sender<Result<()>>),
    GetResponse(ModelingCmdId, oneshot::Sender<Result<OkModelingCmdResponse>>),
    SendModelingBatch(ModelingBatch, oneshot::Sender<Result<()>>),
}

pub async fn start(
    mut incoming: mpsc::Receiver<Request>,
    mut write_to_ws: SplitSink<WebSocketStream<Upgraded>, WsMsg>,
    mut read_from_ws: SplitStream<WebSocketStream<Upgraded>>,
    timeout: Duration,
) {
    let mut responses: HashMap<ModelingCmdId, WebSocketResponse> = HashMap::new();
    'next_request: while let Some(req) = incoming.recv().await {
        match req {
            Request::SendModelingCmd(cmd, responder) => {
                let ws_msg = WsMsg::Text(
                    serde_json::to_string(&WebSocketRequest::ModelingCmdReq(cmd))
                        .expect("ModelingCmdReq can always be serialized"),
                );
                let resp = write_to_ws.send(ws_msg).await.map_err(RunCommandError::WebSocketSend);
                // If the send fails, it's because the caller dropped its end, so ignore the
                // error because we're done with this request anyway.
                let _ = responder.send(resp);
            }
            Request::GetResponse(cmd_id, responder) => {
                let start = Instant::now();
                while start.elapsed() < timeout {
                    // Check the response map.
                    // If we've already got the response for this ID, then send it back to the user!
                    if let Some(resp) = responses.remove(&cmd_id) {
                        let send_this_to_user = match resp {
                            WebSocketResponse::Success(s) => {
                                let resp = s.resp;
                                match resp {
                                    OkWebSocketResponseData::Modeling { modeling_response } => Ok(modeling_response),
                                    _ => {
                                        // This request ID should be for a modeling request. Something's gone very wrong.
                                        Err(RunCommandError::ServerSentWrongType)
                                    }
                                }
                            }
                            WebSocketResponse::Failure(e) => Err(RunCommandError::ModelingApiFailure {
                                request_id: Some(cmd_id.into()),
                                errors: e.errors,
                            }),
                        };
                        // If the send fails, it's because the caller dropped its end, so ignore the
                        // error because we're done with this request anyway.
                        let _ = responder.send(send_this_to_user);
                        // Finished this request! Actor is ready for the next request.
                        continue 'next_request;
                    }
                    // If not, get a response from the WebSocket.
                    // If we can't get any response, the WebSocket must have been closed.
                    let Some(msg) = read_from_ws.next().await else {
                        // If the send fails, it's because the caller dropped its end, so ignore
                        // the error because we're done with this request anyway.
                        let _ = responder.send(Err(RunCommandError::WebSocketClosed));
                        // Probably no point getting another request, but the user may try to,
                        // so we should respect them.
                        continue 'next_request;
                    };
                    // Couldn't read from WebSocket? Try again.
                    let Ok(msg) = msg else {
                        continue;
                    };
                    // WebSocket response wasn't text? Try again.
                    let Some(resp_text) = text_from_ws(msg) else {
                        continue;
                    };
                    // Couldn't decode the response? Try again.
                    let Ok(resp) = decode_websocket_text(&resp_text) else {
                        continue;
                    };
                    if let Some(id) = resp.request_id() {
                        responses.insert(id.into(), resp);
                    } else {
                        continue;
                    }
                }
                // If the send fails, it's because the caller dropped its end, so cancel this request
                // and wait for the next request.
                if responder.send(Err(RunCommandError::TimeOutWaitingForResponse)).is_err() {
                    continue 'next_request;
                }
            }
            Request::SendModelingBatch(batch, responder) => {
                let ws_msg = WsMsg::Text(
                    serde_json::to_string(&WebSocketRequest::ModelingCmdBatchReq(batch))
                        .expect("ModelingCmdReq can always be serialized"),
                );
                let resp = write_to_ws.send(ws_msg).await.map_err(RunCommandError::WebSocketSend);
                // If the send fails, it's because the caller dropped its end, so ignore the
                // error because we're done with this request anyway.
                let _ = responder.send(resp);
            }
        }
    }
}

/// Given the text from a WebSocket, deserialize its JSON.
/// Returns OK if the WebSocket's JSON represents a successful response.
/// Returns an error if the WebSocket's JSON represented a failure response.
fn decode_websocket_text(text: &str) -> Result<WebSocketResponse> {
    let resp = serde_json::from_str(text)?;
    Ok(resp)
}

/// Find the text in a WebSocket message, if there's any.
fn text_from_ws(msg: WsMsg) -> Option<String> {
    match msg {
        WsMsg::Text(text) => Some(text),
        _ => None,
    }
}
