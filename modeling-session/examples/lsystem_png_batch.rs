//! Use the KittyCAD modeling API to draw an L-System and save it to a PNG.
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
    websocket::ModelingCmdReq,
    ClosePath, ExtendPath, Extrude, ModelingCmd, MovePathPen, StartPath, TakeSnapshot,
};
use kittycad_modeling_session::{Session, SessionBuilder};
use lsystem::{LSystem, MapRules};
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Set up the API client.
    let kittycad_api_token = env::var("KITTYCAD_API_TOKEN").context("You must set $KITTYCAD_API_TOKEN")?;
    let kittycad_api_client = kittycad::Client::new(kittycad_api_token);

    // Where should the final PNG be saved?
    let img_output_path = env::var("IMAGE_OUTPUT_PATH").unwrap_or_else(|_| "model_lsystem_batched.png".to_owned());

    let session_builder = SessionBuilder {
        client: kittycad_api_client,
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

    let mut rules = MapRules::new();
    rules.set_str('F', "FF+F+F+F+F+F-F");
    let axiom = "F+F+F+F".chars().collect();
    let mut system = LSystem::new(rules, axiom);

    // Iterates 3 times. More causes PNG snapshot to fail.
    let out_chars = system.nth(2).unwrap();

    // Create a new empty path.
    let path_id = Uuid::new_v4();
    let path = path_id.into();
    session
        .run_command(path, ModelingCmd::from(StartPath::default()))
        .await
        .context("could not create path")?;

    // Add four lines to the path,
    // in the shape of a square.
    // First, start the path at the first corner.
    let mut sketch_batch = vec![ModelingCmdReq {
        cmd_id: random_id(),
        cmd: ModelingCmd::MovePathPen(MovePathPen {
            path,
            to: Point3d {
                x: LengthUnit(0.0),
                y: LengthUnit(0.0),
                z: LengthUnit(0.0),
            },
        }),
    }];

    struct Polar {
        angle: f64,
        length: f64,
        x: f64,
        y: f64,
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let mut angle = 0.0;
    let mut length = 10.0;
    let factor = 1.36;
    let mut stack: Vec<Polar> = vec![];
    let deg = std::f64::consts::PI * 2.0 / 360.0;

    let mut extend_paths: Vec<ModelingCmdReq> = vec![];

    for c in out_chars {
        match c {
            '[' => {
                stack.push(Polar { angle, length, x, y });
            }
            ']' => {
                if let Some(last) = stack.pop() {
                    angle = last.angle;
                    length = last.length;
                    x = last.x;
                    y = last.y;
                };
            }
            '>' => length *= factor,
            '<' => length /= factor,
            '+' => angle = (angle - 90.0) % 360.0,
            '-' => angle = (angle + 90.0) % 360.0,
            'F' => {
                x += (angle * deg).cos() * length;
                y += (angle * deg).sin() * length;

                extend_paths.push(ModelingCmdReq {
                    cmd_id: random_id(),
                    cmd: ModelingCmd::ExtendPath(ExtendPath {
                        path,
                        segment: PathSegment::Line {
                            end: Point3d {
                                x: LengthUnit(x),
                                y: LengthUnit(y),
                                z: LengthUnit(0.0),
                            },
                            relative: false,
                        },
                    }),
                });
            }
            _ => panic!("Unhandled character encountered"),
        }
    }
    sketch_batch.extend(extend_paths);

    sketch_batch.push(ModelingCmdReq {
        cmd: ModelingCmd::ClosePath(ClosePath { path_id }),
        cmd_id: random_id(),
    });
    sketch_batch.push(ModelingCmdReq {
        cmd: ModelingCmd::Extrude(Extrude {
            distance: LengthUnit(1.0),
            target: path,
            faces: None,
            opposite: Default::default(),
            extrude_method: Default::default(),
        }),
        cmd_id: random_id(),
    });
    session
        .run_batch_no_responses(sketch_batch, random_id())
        .await
        .context("could not draw cube in batch")?;

    // Export model as a PNG.
    let snapshot_resp = session
        .run_command(
            random_id(),
            ModelingCmd::from(TakeSnapshot {
                format: kittycad_modeling_cmds::ImageFormat::Png,
            }),
        )
        .await
        .context("could not get PNG snapshot")?;

    // Save the PNG to disk.
    match snapshot_resp {
        OkModelingCmdResponse::TakeSnapshot(snap) => {
            let mut img = image::io::Reader::new(Cursor::new(snap.contents));
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
