//! Use the KittyCAD modeling API to draw a cube and save it to a PNG.
use std::{env, io::Cursor, time::Duration};

use color_eyre::{
    eyre::{bail, Context, Error},
    Result,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use kittycad::types::{
    FailureWebSocketResponse, ModelingCmd, OkModelingCmdResponse, OkWebSocketResponseData, PathSegment, Point3D,
    SuccessWebSocketResponse, WebSocketRequest,
};
use kittycad_modeling_session::{Session, SessionBuilder};
use reqwest::Upgraded;
use tokio::time::timeout;
use tokio_tungstenite::{tungstenite::Message as WsMsg, WebSocketStream};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the API client.
    let kittycad_api_token = env::var("KITTYCAD_API_TOKEN").context("You must set $KITTYCAD_API_TOKEN")?;
    let kittycad_api_client = kittycad::Client::new(kittycad_api_token);

    // Where should the final PNG be saved?
    let img_output_path = env::var("IMAGE_OUTPUT_PATH").unwrap_or_else(|_| "model.png".to_owned());

    let session_builder = SessionBuilder {
        client: kittycad_api_client,
        fps: Some(10),
        unlocked_framerate: Some(false),
        video_res_height: Some(720),
        video_res_width: Some(1280),
    };
    let session = Session::start(session_builder).await?;

    // First, send all commands to the API, to draw a cube.
    // Then, read all responses from the API, to download the cube as a PNG.
    // draw_cube(write, 10.0).await?;
    // export_png(read, img_output_path).await
    Ok(())
}

/// Send modeling commands to the KittyCAD API.
/// We're going to draw a cube and export it as a PNG.
async fn draw_cube(mut write_to_ws: SplitSink<WebSocketStream<Upgraded>, WsMsg>, width: f64) -> Result<()> {
    // All messages to the KittyCAD Modeling API will be sent over the WebSocket as Text.
    // The text will contain JSON representing a `ModelingCmdReq`.
    // This takes in a command and its ID, and makes a WebSocket message containing that command.
    fn to_msg(cmd: ModelingCmd, cmd_id: Uuid) -> WsMsg {
        WsMsg::Text(serde_json::to_string(&WebSocketRequest::ModelingCmdReq { cmd, cmd_id }).unwrap())
    }

    // Now the WebSocket is set up and ready to use!
    // We can start sending commands.

    // Create a new empty path.
    let path_id = Uuid::new_v4();
    write_to_ws.send(to_msg(ModelingCmd::StartPath {}, path_id)).await?;

    // Add four lines to the path,
    // in the shape of a square.
    // First, start the path at the first corner.
    let start = Point3D {
        x: -width,
        y: -width,
        z: -width,
    };
    write_to_ws
        .send(to_msg(
            ModelingCmd::MovePathPen {
                path: path_id,
                to: start.clone(),
            },
            Uuid::new_v4(),
        ))
        .await?;

    // Now extend the path to each corner, and back to the start.
    let points = [
        Point3D {
            x: width,
            y: -width,
            z: -width,
        },
        Point3D {
            x: width,
            y: width,
            z: -width,
        },
        Point3D {
            x: -width,
            y: width,
            z: -width,
        },
        start,
    ];
    for point in points {
        write_to_ws
            .send(to_msg(
                ModelingCmd::ExtendPath {
                    path: path_id,
                    segment: PathSegment::Line {
                        end: point,
                        relative: false,
                    },
                },
                Uuid::new_v4(),
            ))
            .await?;
    }

    // Extrude the square into a cube.
    write_to_ws
        .send(to_msg(ModelingCmd::ClosePath { path_id }, Uuid::new_v4()))
        .await?;
    write_to_ws
        .send(to_msg(
            ModelingCmd::Extrude {
                cap: true,
                distance: width * 2.0,
                target: path_id,
            },
            Uuid::new_v4(),
        ))
        .await?;

    // Export the model as a PNG.
    write_to_ws
        .send(to_msg(
            ModelingCmd::TakeSnapshot {
                format: kittycad::types::ImageFormat::Png,
            },
            Uuid::new_v4(),
        ))
        .await?;

    // Finish sending
    drop(write_to_ws);
    Ok(())
}

fn save_image(contents: Vec<u8>, output_path: &str) -> Result<()> {
    let mut img = image::io::Reader::new(Cursor::new(contents));
    img.set_format(image::ImageFormat::Png);
    let img = img.decode()?;
    img.save(output_path)?;
    Ok(())
}
