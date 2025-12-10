//! Use the KittyCAD modeling API to draw a cube and save it to a PNG.
use std::{env, io::Cursor};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use kittycad_modeling_cmds::{
    id::ModelingCmdId,
    length_unit::LengthUnit,
    ok_response::OkModelingCmdResponse,
    shared::{PathSegment, Point3d},
    ClosePath, ExtendPath, Extrude, ModelingCmd, MovePathPen, StartPath, TakeSnapshot,
};
use kittycad_modeling_session::{Session, SessionBuilder};
use uuid::Uuid;

const CUBE_WIDTH: LengthUnit = LengthUnit(100.0);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Set up the API client.
    let token = env::var("ZOO_API_TOKEN")
        .or_else(|_| env::var("KITTYCAD_API_TOKEN")) // legacy name
        .context("You must set $ZOO_API_TOKEN")?;
    let client = kittycad::Client::new(token);

    // Where should the final PNG be saved?
    let img_output_path = env::var("IMAGE_OUTPUT_PATH").unwrap_or_else(|_| "model.png".to_owned());

    let session_builder = SessionBuilder {
        client,
        fps: Some(10),
        unlocked_framerate: Some(false),
        video_res_height: Some(720),
        video_res_width: Some(1280),
        buffer_reqs: None,
        await_response_timeout: None,
        show_grid: None,
    };
    let mut session = Session::start(session_builder)
        .await
        .context("could not establish session")?;

    // Create a new empty path.
    let path_id = Uuid::new_v4();
    let path = path_id.into();
    session
        .run_command(path, ModelingCmd::StartPath(StartPath::default()))
        .await
        .context("could not create path")?;

    // Add four lines to the path,
    // in the shape of a square.
    // First, start the path at the first corner.
    let start = Point3d {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -CUBE_WIDTH,
    };
    session
        .run_command(random_id(), MovePathPen { path, to: start }.into())
        .await
        .context("could not move path pen to start")?;

    // Now extend the path to each corner, and back to the start.
    let points = [
        Point3d {
            x: CUBE_WIDTH,
            y: -CUBE_WIDTH,
            z: -CUBE_WIDTH,
        },
        Point3d {
            x: CUBE_WIDTH,
            y: CUBE_WIDTH,
            z: -CUBE_WIDTH,
        },
        Point3d {
            x: -CUBE_WIDTH,
            y: CUBE_WIDTH,
            z: -CUBE_WIDTH,
        },
        start,
    ];
    for point in points {
        session
            .run_command(
                random_id(),
                ExtendPath {
                    path,
                    segment: PathSegment::Line {
                        end: point,
                        relative: false,
                    },
                    label: Default::default(),
                }
                .into(),
            )
            .await
            .context("could not draw square")?;
    }
    // Extrude the square into a cube.
    session
        .run_command(random_id(), ModelingCmd::ClosePath(ClosePath { path_id }))
        .await
        .context("could not close square path")?;
    session
        .run_command(
            random_id(),
            Extrude {
                distance: CUBE_WIDTH * 2.0,
                target: path,
                faces: None,
                opposite: Default::default(),
                extrude_method: Default::default(),
                merge_coplanar_faces: Default::default(),
                body_type: Default::default(),
            }
            .into(),
        )
        .await
        .context("could not extrude square into cube")?;
    // Export model as a PNG.
    let snapshot_resp = session
        .run_command(
            random_id(),
            TakeSnapshot {
                format: kittycad_modeling_cmds::ImageFormat::Png,
            }
            .into(),
        )
        .await
        .context("could not get PNG snapshot")?;

    // Save the PNG to disk.
    match snapshot_resp {
        OkModelingCmdResponse::TakeSnapshot(snap) => {
            let mut img = image::ImageReader::new(Cursor::new(snap.contents));
            img.set_format(image::ImageFormat::Png);
            let img = img.decode().context("could not decode PNG bytes")?;
            img.save(img_output_path).context("could not save PNG to disk")?;
        }
        other => bail!("Unexpected response: {other:?}"),
    };
    Ok(())
}

fn random_id() -> ModelingCmdId {
    Uuid::new_v4().into()
}
