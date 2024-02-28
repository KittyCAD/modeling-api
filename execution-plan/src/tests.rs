use std::env;

use insta::assert_snapshot;
use kittycad_execution_plan_traits::{InMemory, ListHeader, ObjectHeader, Primitive, Value};
use kittycad_modeling_cmds::ModelingCmdEndpoint as Endpoint;
use kittycad_modeling_cmds::{
    id::ModelingCmdId,
    length_unit::LengthUnit,
    shared::{PathSegment, Point3d, Point4d},
};
use kittycad_modeling_session::{Session, SessionBuilder};
use uuid::Uuid;

use crate::{arithmetic::operator::BinaryOperation, Address};

use super::*;

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
    match Session::start(session_builder).await {
        Err(e) => match e {
            kittycad::types::error::Error::InvalidRequest(s) => panic!("Request did not meet requirements {s}"),
            kittycad::types::error::Error::CommunicationError(e) => {
                panic!(" A server error either due to the data, or with the connection: {e}")
            }
            kittycad::types::error::Error::RequestError(e) => panic!("Could not build request: {e}"),
            kittycad::types::error::Error::SerdeError { error, status } => {
                panic!("Serde error (HTTP {status}): {error}")
            }
            kittycad::types::error::Error::InvalidResponsePayload { error, response } => {
                panic!("Invalid response payload. Error {error}, response {response:?}")
            }
            kittycad::types::error::Error::Server { body, status } => panic!("Server error (HTTP {status}): {body}"),
            kittycad::types::error::Error::UnexpectedResponse(resp) => {
                let status = resp.status();
                let url = resp.url().to_owned();
                match resp.text().await {
                    Ok(body) => panic!(
                        "Unexpected response from KittyCAD API.
                        URL:{url}
                        HTTP {status}
                        ---Body----
                        {body}"
                    ),
                    Err(e) => panic!(
                        "Unexpected response from KittyCAD API.
                        URL:{url}
                        HTTP {status}
                        ---Body could not be read, the error is----
                        {e}"
                    ),
                }
            }
        },
        Ok(x) => x,
    }
}

#[tokio::test]
async fn write_addr_to_memory() {
    let plan = vec![Instruction::SetPrimitive {
        address: Address::ZERO,
        value: 3.4.into(),
    }];
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address::ZERO), Some(&3.4.into()))
}

#[tokio::test]
async fn add_literals() {
    let plan = vec![Instruction::BinaryArithmetic {
        arithmetic: BinaryArithmetic {
            operation: BinaryOperation::Add,
            operand0: Operand::Literal(3u32.into()),
            operand1: Operand::Literal(2u32.into()),
        },
        destination: Destination::Address(Address::ZERO + 1),
    }];
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&5u32.into()))
}

#[tokio::test]
async fn basic_stack() {
    let plan = vec![
        Instruction::StackPush {
            data: vec![33u32.into()],
        },
        Instruction::StackPop {
            destination: Some(Address::ZERO),
        },
    ];
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address::ZERO), Some(&33u32.into()));
    assert!(mem.stack.is_empty());
}

#[tokio::test]
async fn add_stack() {
    let plan = vec![
        Instruction::StackPush {
            data: vec![10u32.into()],
        },
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Add,
                operand0: Operand::Literal(20u32.into()),
                operand1: Operand::StackPop,
            },
            destination: Destination::Address(Address::ZERO),
        },
    ];
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&Address::ZERO), Some(&30u32.into()))
}

#[tokio::test]
async fn add_literal_to_reference() {
    let plan = vec![
        // Memory addr 0 contains 450
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: 450u32.into(),
        },
        // Add 20 to addr 0
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Add,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal(20u32.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // 20 + 450 = 470
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&470u32.into()))
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
    let start_addr = Address::ZERO;
    mem.set_composite(start_addr, point_before);
    assert_eq!(mem.get(&Address::ZERO), Some(&(2.0.into())));
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&(3.0.into())));
    assert_eq!(mem.get(&(Address::ZERO + 2)), Some(&(4.0.into())));

    // Update the point's x-value in memory.
    execute(
        &mut mem,
        vec![Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Add,
                operand0: Operand::Reference(start_addr),
                operand1: Operand::Literal(40u32.into()),
            },
            destination: Destination::Address(start_addr),
        }],
        None,
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
async fn modulo_and_power_with_reference() {
    // Modulo with two positive integers
    let plan = vec![
        // Memory addr 0 contains 450
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: 450u32.into(),
        },
        // Take (address 0) % 20
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Mod,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal(20u32.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // 450 % 20 = 10
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&10u32.into()));

    // Pow with a positive integer and a positive float
    let plan = vec![
        // Memory addr 0 contains 2.5
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: 2.5f32.into(),
        },
        // Take (address 0) ^ 2
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Pow,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal(2u32.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // 2.5^2 = 6.25
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&6.25f32.into()));

    // Modulo with two positive floats
    let plan = vec![
        // Memory addr 0 contains 12.5
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: 12.5f32.into(),
        },
        // Take (address 0) % 2.25
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Mod,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal(2.25f32.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // 12.5 % 2.25 = 1.25
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&1.25f32.into()));

    // Pow with a two negative floats
    let plan = vec![
        // Memory addr 0 contains -2.5
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: (-2.5f32).into(),
        },
        // Take (address 0) ^ -4.2
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Pow,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal((-4.2f32).into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // (-2.5)^-4.2 = NaN
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    let result: f32 = mem.get_primitive(&(Address::ZERO + 1)).unwrap();
    assert!(result.is_nan());

    // Modulo with two negative integers
    let plan = vec![
        // Memory addr 0 contains -450
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: (-450i64).into(),
        },
        // Take (address 0) % -20
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Mod,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal((-20i64).into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // -450 % -20 = -10
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&(-10i64).into()));

    // Modulo with a negative integer and a positive integer
    let plan = vec![
        // Memory addr 0 contains -450
        Instruction::SetPrimitive {
            address: Address::ZERO,
            value: (-450i64).into(),
        },
        // Take (address 0) % 20
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Mod,
                operand0: Operand::Reference(Address::ZERO),
                operand1: Operand::Literal(20u32.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        },
    ];
    // -450 % 20 = -10
    let mut mem = Memory::default();
    execute(&mut mem, plan, None).await.expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&(-10i64).into()));
}

#[tokio::test]
async fn get_element_of_array() {
    let mut mem = Memory::default();
    const START_DATA_AT: usize = 10;
    let point_4d = Point4d {
        x: 20.0f64,
        y: 21.0,
        z: 22.0,
        w: 23.0,
    };
    let list = vec![
        Point3d {
            x: 12.0f64,
            y: 13.0,
            z: 14.0,
        }
        .into_parts(),
        point_4d.into_parts(),
    ];
    execute(
        &mut mem,
        vec![
            Instruction::SetList {
                start: START_DATA_AT.into(),
                elements: list,
            },
            Instruction::AddrOfMember {
                start: Operand::Literal(Primitive::Address(Address::ZERO + 10)),
                member: Operand::Literal(Primitive::from(1usize)),
            },
        ],
        None,
    )
    .await
    .unwrap();
    assert_snapshot!("set_array_memory", mem.debug_table(None));

    // The last instruction put the 4D point (element 1) on the stack.
    // Check it's there.
    let actual = mem.stack.pop().unwrap();
    assert_eq!(actual, vec![(Address::ZERO + 15).into()]);

    // The memory should start with a list header.
    let ListHeader { count: _, size } = mem.get_primitive(&(Address::ZERO + START_DATA_AT)).unwrap();
    // Check the edge of `size`.
    assert!(mem.get(&(Address::ZERO + START_DATA_AT + size)).is_some());
    assert!(mem.get(&(Address::ZERO + START_DATA_AT + size + 1)).is_none());
}

#[tokio::test]
async fn copy_len() {
    // Initialize memory with a single object:
    // { first: <3d point>, second: <4d point> }
    let mut smem = StaticMemoryInitializer::default();
    let size_of_object = 9;
    smem.push(Primitive::from(ObjectHeader {
        properties: vec!["first".to_owned(), "second".to_owned()],
        size: size_of_object,
    }));
    smem.push(Primitive::from(3usize));
    smem.push(Point3d {
        x: 12.0f64,
        y: 13.0,
        z: 14.0,
    });
    smem.push(Primitive::from(4usize));
    smem.push(Point4d {
        x: 20.0f64,
        y: 21.0,
        z: 22.0,
        w: 23.0,
    });
    let mut mem = smem.finish();

    // Addr 5 is a property of the object at addr 0.
    // Its key is "second" and its value is a Point4d.
    // The property is preceded by its length (4) because that's just how KCEP stores objects.
    // Push that address onto the stack.
    let start_of_second_property = Address::ZERO + 5;
    mem.stack.push(vec![(start_of_second_property).into()]);

    // Copy the value at addr 5 into addr 100.
    let copied_into = Address::ZERO + 100;
    execute(
        &mut mem,
        vec![Instruction::CopyLen {
            source_range: Operand::StackPop,
            destination_range: Operand::Literal(Primitive::Address(copied_into)),
        }],
        None,
    )
    .await
    .unwrap();

    // Assert that the property was properly copied into the destination.
    assert_eq!(mem.get(&copied_into), mem.get(&(start_of_second_property + 1)));
    assert_eq!(mem.get(&(copied_into + 1)), mem.get(&(start_of_second_property + 2)));
    assert_eq!(mem.get(&(copied_into + 2)), mem.get(&(start_of_second_property + 3)));
}

#[tokio::test]
async fn get_key_of_object() {
    let point_4d = Point4d {
        x: 20.0f64,
        y: 21.0,
        z: 22.0,
        w: 23.0,
    };
    let point_3d = Point3d {
        x: 12.0f64,
        y: 13.0,
        z: 14.0,
    };
    let mut smem = StaticMemoryInitializer::default();
    let size = 9;
    let start = smem.push(Primitive::from(ObjectHeader {
        properties: vec!["first".to_owned(), "second".to_owned()],
        size,
    }));
    smem.push(Primitive::from(3usize));
    smem.push(point_3d);
    smem.push(Primitive::from(4usize));
    smem.push(point_4d);
    let mut mem = smem.finish();
    execute(
        &mut mem,
        vec![Instruction::AddrOfMember {
            start: Operand::Literal(Primitive::Address(start)),
            member: Operand::Literal("second".to_owned().into()),
        }],
        None,
    )
    .await
    .unwrap();

    // The last instruction put the 4D point (element 1) on the stack.
    // Check it's there.
    let actual = mem.stack.pop().unwrap();
    assert_eq!(actual, vec![(Address::ZERO + 5).into()]);
}

#[tokio::test]
async fn api_call_draw_cube() {
    let client = test_client().await;

    const CUBE_WIDTH: LengthUnit = LengthUnit(200.0);

    // Define primitives, load them into memory.
    let mut static_data = StaticMemoryInitializer::default();
    let path = ModelingCmdId(Uuid::parse_str("4cd175a3-e313-4c91-b624-368bea3c0483").unwrap());
    let path_id_addr = static_data.push(Primitive::from(path.0));
    let cube_height_addr = static_data.push(Primitive::from(CUBE_WIDTH * 2.0));
    let cap_addr = static_data.push(Primitive::from(true));
    let img_format_addr = static_data.push(Primitive::from("Png".to_owned()));
    let output_addr = Address::ZERO + 99;
    let starting_point = Point3d {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -CUBE_WIDTH,
    };
    let line_segment = |end: Point3d<LengthUnit>| PathSegment::Line { end, relative: false };
    let segments = [
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
        starting_point,
    ]
    .map(line_segment);
    let segment_addrs = segments.map(|segment| static_data.push(segment));
    let mut mem = static_data.finish();
    assert_snapshot!("cube_memory_before", mem.debug_table(None));

    // Run the plan!
    execute(
        &mut mem,
        vec![
            Instruction::StackPush {
                data: starting_point.into_parts(),
            },
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
                arguments: vec![InMemory::Address(path_id_addr), InMemory::StackPop],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr.into(), segment_addrs[0].into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr.into(), segment_addrs[1].into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr.into(), segment_addrs[2].into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ExtendPath,
                store_response: None,
                arguments: vec![path_id_addr.into(), segment_addrs[3].into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ClosePath,
                store_response: None,
                arguments: vec![path_id_addr.into()],
                cmd_id: new_id(),
            }),
            // Turn square into cube
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::Extrude,
                store_response: None,
                arguments: vec![path_id_addr.into(), cube_height_addr.into(), cap_addr.into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::TakeSnapshot,
                store_response: Some(output_addr),
                arguments: vec![img_format_addr.into()],
                cmd_id: new_id(),
            }),
        ],
        Some(client),
    )
    .await
    .unwrap();

    // Program executed successfully!
    assert_snapshot!("cube_memory_after", mem.debug_table(None));

    // The image output was set to addr 99.
    // Outputs are two addresses long, addr 99 will store the data format (TAKE_SNAPSHOT)
    // and addr 100 will store its first field ('contents', the image bytes).
    let Primitive::Bytes(b) = mem.get(&(Address::ZERO + 100)).as_ref().unwrap() else {
        panic!("wrong format in memory addr 100");
    };
    // Visually check that the image is a cube.
    use image::io::Reader as ImageReader;
    let img = ImageReader::new(std::io::Cursor::new(b))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    twenty_twenty::assert_image("tests/outputs/cube.png", &img, 0.9999);
}

fn new_id() -> ModelingCmdId {
    ModelingCmdId(Uuid::new_v4())
}
