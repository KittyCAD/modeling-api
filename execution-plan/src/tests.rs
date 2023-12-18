use std::env;

use insta::assert_snapshot;
use kittycad_modeling_cmds::shared::{PathSegment, Point3d};
use kittycad_modeling_session::{Session, SessionBuilder};
use tabled::{settings::Style, Table};
use uuid::Uuid;

use super::*;
use crate::primitive::NumericPrimitive;

async fn test_client() -> Session {
    let kittycad_api_token = env::var("KITTYCAD_API_TOKEN").expect("You must set $KITTYCAD_API_TOKEN");
    let kittycad_api_client = kittycad::Client::new(kittycad_api_token);
    let session_builder = SessionBuilder {
        client: kittycad_api_client,
        fps: Some(10),
        unlocked_framerate: Some(false),
        video_res_height: Some(720),
        video_res_width: Some(1280),
        buffer_reqs: None,
        await_response_timeout: None,
    };
    Session::start(session_builder)
        .await
        .expect("could not connect to KittyCAD engine")
}

#[tokio::test]
async fn write_addr_to_memory() {
    let plan = vec![Instruction::Set {
        address: Address(0),
        value: 3.4.into(),
    }];
    let mut mem = Memory::default();
    let client = test_client().await;
    execute(&mut mem, plan, client).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address(0)), Some(&3.4.into()))
}

#[tokio::test]
async fn add_literals() {
    let plan =
        vec![Instruction::Arithmetic {
            arithmetic: Arithmetic {
                operation: Operation::Add,
                operand0: Operand::Literal(3.into()),
                operand1: Operand::Literal(2.into()),
            },
            destination: Address(1),
        }];
    let mut mem = Memory::default();
    let client = test_client().await;
    execute(&mut mem, plan, client).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address(1)), Some(&5.into()))
}

#[tokio::test]
async fn add_literal_to_reference() {
    let plan = vec![
        // Memory addr 0 contains 450
        Instruction::Set {
            address: Address(0),
            value: 450.into(),
        },
        // Add 20 to addr 0
        Instruction::Arithmetic {
            arithmetic: Arithmetic {
                operation: Operation::Add,
                operand0: Operand::Reference(Address(0)),
                operand1: Operand::Literal(20.into()),
            },
            destination: Address(1),
        },
    ];
    // 20 + 450 = 470
    let mut mem = Memory::default();
    let client = test_client().await;
    execute(&mut mem, plan, client).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address(1)), Some(&470.into()))
}

#[tokio::test]
async fn add_to_composite_value() {
    let mut mem = Memory::default();

    // Write a point to memory.
    let point_before = Point3d {
        x: 2.0f64,
        y: 3.0,
        z: 4.0,
    };
    let start_addr = Address(0);
    mem.set_composite(start_addr, point_before);
    assert_eq!(mem.get(&Address(0)), Some(&(2.0.into())));
    assert_eq!(mem.get(&Address(1)), Some(&(3.0.into())));
    assert_eq!(mem.get(&Address(2)), Some(&(4.0.into())));

    let client = test_client().await;
    // Update the point's x-value in memory.
    execute(
        &mut mem,
        vec![Instruction::Arithmetic {
            arithmetic: Arithmetic {
                operation: Operation::Add,
                operand0: Operand::Reference(start_addr),
                operand1: Operand::Literal(40.into()),
            },
            destination: start_addr,
        }],
        client,
    )
    .await
    .unwrap();

    // Read the point out of memory, validate it.
    let point_after: Point3d<f64> = mem.get_composite(start_addr).unwrap();
    assert_eq!(
        point_after,
        Point3d {
            x: 42.0,
            y: 3.0,
            z: 4.0
        }
    )
}

#[tokio::test]
async fn api_call_draw_cube() {
    let mut mem = Memory::default();
    let client = test_client().await;

    const CUBE_WIDTH: f64 = 20.0;

    // Define primitives, load them into memory.
    let path_id_addr = Address(0);
    let path = ModelingCmdId(Uuid::parse_str("4cd175a3-e313-4c91-b624-368bea3c0483").unwrap());
    let cube_height_addr = Address(2);
    let cube_height = Primitive::from(CUBE_WIDTH * 2.0);
    let cap_addr = Address(3);
    let cap = Primitive::Bool(true);
    let img_format_addr = Address(4);
    let img_format = Primitive::from("png".to_owned());
    let output_addr = Address(99);
    mem.set(path_id_addr, Primitive::from(path.0));
    mem.set(cube_height_addr, cube_height);
    mem.set(cap_addr, cap);
    mem.set(img_format_addr, img_format);

    // Define composite objects, load them into memory.
    let starting_point_addr = Address(6);
    let starting_point = Point3d {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -CUBE_WIDTH,
    };
    let point_size = mem.set_composite(starting_point_addr, starting_point);
    let next_addr = Address(starting_point_addr.0 + point_size);
    let segments =
        [
            PathSegment::Line {
                end: Point3d {
                    x: CUBE_WIDTH,
                    y: -CUBE_WIDTH,
                    z: -CUBE_WIDTH,
                },
                relative: false,
            },
            PathSegment::Line {
                end: Point3d {
                    x: CUBE_WIDTH,
                    y: CUBE_WIDTH,
                    z: -CUBE_WIDTH,
                },
                relative: false,
            },
            PathSegment::Line {
                end: Point3d {
                    x: -CUBE_WIDTH,
                    y: CUBE_WIDTH,
                    z: -CUBE_WIDTH,
                },
                relative: false,
            },
            PathSegment::Line {
                end: starting_point,
                relative: false,
            },
        ];
    let mut segment_addrs = vec![next_addr];
    for segment in segments {
        let addr = segment_addrs.last().unwrap();
        let size = mem.set_composite(*addr, segment);
        segment_addrs.push(Address(addr.0 + size));
    }
    assert_snapshot!("cube_memory_before", debug_dump_memory(&mem));

    // Run the plan!
    execute(
        &mut mem,
        vec![
            // Start the path.
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::StartPath,
                store_response: None,
                arguments: vec![],
                cmd_id: path,
            }),
            // Draw a square.
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::MovePathPen,
                store_response: None,
                arguments: vec![path_id_addr, starting_point_addr],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr, segment_addrs[0]],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr, segment_addrs[1]],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr, segment_addrs[2]],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr, segment_addrs[3]],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ClosePath,
                store_response: None,
                arguments: vec![path_id_addr],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::Extrude,
                store_response: None,
                arguments: vec![path_id_addr, cube_height_addr, cap_addr],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::TakeSnapshot,
                store_response: Some(output_addr),
                arguments: vec![img_format_addr],
                cmd_id: new_id(),
            }),
        ],
        client,
    )
    .await
    .unwrap();

    // Program executed successfully!
    assert_snapshot!("cube_memory_after", debug_dump_memory(&mem));

    // The image output was set to addr 99.
    // Outputs are two addresses long, addr 99 will store the data format (TAKE_SNAPSHOT)
    // and addr 100 will store its first field ('contents', the image bytes).
    let Primitive::Bytes(b) = mem.get(&Address(100)).as_ref().unwrap() else {
        panic!("wrong format in memory addr 100");
    };
    // Visually check that the image is a cube.
    use image::io::Reader as ImageReader;
    let img =
        ImageReader::new(std::io::Cursor::new(b))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    twenty_twenty::assert_image("tests/outputs/cube.png", &img, 0.9999);
}

/// Return a nicely-formatted table of memory.
fn debug_dump_memory(mem: &Memory) -> String {
    impl Primitive {
        fn pretty_print(&self) -> (&'static str, String) {
            match self {
                Primitive::String(v) => ("String", v.to_owned()),
                Primitive::NumericValue(NumericPrimitive::Float(v)) => ("Float", v.to_string()),
                Primitive::NumericValue(NumericPrimitive::Integer(v)) => ("Integer", v.to_string()),
                Primitive::Uuid(v) => ("Uuid", v.to_string()),
                Primitive::Bytes(v) => ("Bytes", format!("length {}", v.len())),
                Primitive::Bool(v) => ("Bool", v.to_string()),
                Primitive::Nil => ("Nil", String::new()),
            }
        }
    }
    #[derive(tabled::Tabled)]
    struct MemoryAddr {
        index: usize,
        val_type: &'static str,
        value: String,
    }
    let table_data: Vec<_> =
        mem.addresses
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if let Some(val) = val {
                    let (val_type, value) = val.pretty_print();
                    Some(MemoryAddr {
                        index: i,
                        val_type,
                        value,
                    })
                } else {
                    None
                }
            })
            .collect();
    Table::new(table_data).with(Style::sharp()).to_string()
}

fn new_id() -> ModelingCmdId {
    ModelingCmdId(Uuid::new_v4())
}
