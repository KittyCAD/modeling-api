use std::env;

use insta::assert_snapshot;
use kittycad_execution_plan_traits::{InMemory, ListHeader, ObjectHeader, Primitive, Value};
use kittycad_modeling_cmds::shared::Point2d;
use kittycad_modeling_cmds::ModelingCmdEndpoint as Endpoint;
use kittycad_modeling_cmds::{
    coord,
    id::ModelingCmdId,
    length_unit::LengthUnit,
    shared::{PathSegment, Point3d, Point4d},
};
use kittycad_modeling_session::{Session, SessionBuilder};
use uuid::Uuid;

use crate::sketch_types::{Axes, BasePath, Plane, SketchGroup};
use crate::{
    arithmetic::operator::BinaryOperation, arithmetic::operator::UnaryOperation, constants, Address, Destination,
};

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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&5u32.into()))
}

#[tokio::test]
async fn min_uint_uint() {
    let plan = vec![Instruction::BinaryArithmetic {
        arithmetic: BinaryArithmetic {
            operation: BinaryOperation::Min,
            operand0: Operand::Literal(1u32.into()),
            operand1: Operand::Literal(2u32.into()),
        },
        destination: Destination::Address(Address::ZERO + 1),
    }];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(*mem.get(&(Address::ZERO + 1)).unwrap(), 1u32.into())
}

#[tokio::test]
async fn log_float_float() {
    let plan = vec![Instruction::BinaryArithmetic {
        arithmetic: BinaryArithmetic {
            operation: BinaryOperation::Log,
            operand0: Operand::Literal(100f64.into()),
            operand1: Operand::Literal(10f64.into()),
        },
        destination: Destination::Address(Address::ZERO + 1),
    }];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(*mem.get(&(Address::ZERO + 1)).unwrap(), 2f64.into())
}

#[tokio::test]
async fn max_uint_uint() {
    let plan = vec![Instruction::BinaryArithmetic {
        arithmetic: BinaryArithmetic {
            operation: BinaryOperation::Max,
            operand0: Operand::Literal(1u32.into()),
            operand1: Operand::Literal(2u32.into()),
        },
        destination: Destination::Address(Address::ZERO + 1),
    }];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(*mem.get(&(Address::ZERO + 1)).unwrap(), 2u32.into())
}

#[tokio::test]
async fn pop_off_stack_into_stack() {
    // Test that StackPop works when its destination is StackExtend.
    let plan = vec![
        Instruction::StackPush {
            data: vec![4u32.into()],
        },
        Instruction::StackPush {
            data: vec![5u32.into()],
        },
        Instruction::StackPop {
            destination: Some(Destination::StackExtend),
        },
    ];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(mem.stack_pop().unwrap(), vec![4u32.into(), 5u32.into()]);
}

#[tokio::test]
async fn pop_off_stack_no_op() {
    // Popping off a stack back onto the stack should be a no-op.
    let plan = vec![
        Instruction::StackPush {
            data: vec![4u32.into()],
        },
        Instruction::StackPop {
            destination: Some(Destination::StackPush),
        },
    ];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(mem.stack_pop().unwrap(), vec![4u32.into()]);
}

#[tokio::test]
async fn basic_stack() {
    let plan = vec![
        Instruction::StackPush {
            data: vec![33u32.into()],
        },
        Instruction::StackPop {
            destination: Some(Destination::Address(Address::ZERO)),
        },
    ];
    let mut mem = Memory::default();
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
        &mut None,
    )
    .await
    .unwrap();

    // Read the point out of memory, validate it.
    let (point_after, count): (Point3d<f64>, _) = mem.get_composite(start_addr).unwrap();
    assert_eq!(count, 3);
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
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
    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");
    assert_eq!(mem.get(&(Address::ZERO + 1)), Some(&(-10i64).into()));
}

#[tokio::test]
async fn add_path_to_sketch_group() {
    let mut mem = Memory::default();
    let axes = Axes {
        x: Point3d { x: 1.0, y: 0.0, z: 0.0 },
        y: Point3d { x: 0.0, y: 1.0, z: 0.0 },
        z: Point3d { x: 0.0, y: 0.0, z: 1.0 },
    };
    let sg = SketchGroup {
        id: Uuid::new_v4(),
        on: sketch_types::SketchSurface::Plane(Plane {
            id: Uuid::new_v4(),
            value: sketch_types::PlaneType::XY,
            origin: Default::default(),
            axes,
        }),
        position: Default::default(),
        rotation: Point4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
        axes,
        entity_id: None,
        path_first: BasePath {
            from: Default::default(),
            to: Point2d { x: 20.0, y: 30.0 },
            name: "first".to_owned(),
        },
        path_rest: vec![crate::sketch_types::PathSegment::ToPoint {
            base: BasePath {
                from: Point2d { x: 20.0, y: 30.0 },
                to: Point2d { x: 20.0, y: 0.0 },
                name: "second".to_owned(),
            },
        }],
    };
    let next = sketch_types::PathSegment::ToPoint {
        base: BasePath {
            from: Point2d { x: 20.0, y: 0.0 },
            to: Point2d::default(),
            name: "third".to_owned(),
        },
    };
    let instructions = vec![
        Instruction::SketchGroupSet {
            sketch_group: sg,
            destination: 0,
        },
        Instruction::StackPush {
            data: next.clone().into_parts(),
        },
        Instruction::SketchGroupAddSegment {
            segment: InMemory::StackPop,
            source: 0,
            destination: 0,
        },
    ];
    execute(&mut mem, instructions, &mut None).await.unwrap();
    assert_eq!(mem.sketch_groups[0].path_rest.last().unwrap(), &next);
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
        &mut None,
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
        &mut None,
    )
    .await
    .unwrap();

    // Assert that the property was properly copied into the destination.
    assert_eq!(mem.get(&copied_into), mem.get(&(start_of_second_property + 1)));
    assert_eq!(mem.get(&(copied_into + 1)), mem.get(&(start_of_second_property + 2)));
    assert_eq!(mem.get(&(copied_into + 2)), mem.get(&(start_of_second_property + 3)));
}

#[tokio::test]
async fn copy_onto_addresses() {
    let mut mem = Memory::default();
    execute(
        &mut mem,
        vec![
            Instruction::SetPrimitive {
                address: Address::ZERO + 3,
                value: 1.0.into(),
            },
            Instruction::SetPrimitive {
                address: Address::ZERO + 4,
                value: 2.0.into(),
            },
            Instruction::SetPrimitive {
                address: Address::ZERO + 5,
                value: 3.0.into(),
            },
            Instruction::Copy {
                source: Address::ZERO + 4,
                length: 2,
                destination: Destination::Address(Address::ZERO + 10),
            },
        ],
        &mut None,
    )
    .await
    .unwrap();
    assert_eq!(
        mem.get_slice(Address::ZERO + 10, 2).unwrap(),
        vec![2.0.into(), 3.0.into()]
    );
}
#[tokio::test]
async fn copy_onto_stack() {
    let mut mem = Memory::default();
    execute(
        &mut mem,
        vec![
            Instruction::SetPrimitive {
                address: Address::ZERO + 3,
                value: 1.0.into(),
            },
            Instruction::SetPrimitive {
                address: Address::ZERO + 4,
                value: 2.0.into(),
            },
            Instruction::SetPrimitive {
                address: Address::ZERO + 5,
                value: 3.0.into(),
            },
            Instruction::Copy {
                source: Address::ZERO + 4,
                length: 2,
                destination: Destination::StackPush,
            },
        ],
        &mut None,
    )
    .await
    .unwrap();
    assert_eq!(mem.stack.pop().unwrap(), vec![2.0.into(), 3.0.into()]);
}

#[tokio::test]
async fn stack_extend() {
    let mut mem = Memory::default();
    execute(
        &mut mem,
        vec![
            Instruction::StackPush {
                data: vec![Primitive::Nil],
            },
            Instruction::StackExtend {
                data: vec![Primitive::Bool(true)],
            },
        ],
        &mut None,
    )
    .await
    .unwrap();
    assert_eq!(mem.stack.pop().unwrap(), vec![Primitive::Nil, Primitive::Bool(true)]);
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
        &mut None,
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
        &mut Some(client),
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

macro_rules! test_unary_op {
    ($op:ident, $i:expr, $o:expr) => {
        let plan = vec![Instruction::UnaryArithmetic {
            arithmetic: UnaryArithmetic {
                operation: UnaryOperation::$op,
                operand: Operand::Literal($i.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        }];

        let mut mem = Memory::default();

        execute(&mut mem, plan, &mut None)
            .await
            .expect("failed to execute plan");

        assert_eq!(*mem.get(&(Address::ZERO + 1)).unwrap(), ($o).into())
    };
}

macro_rules! test_unary_op_intentional_err {
    ($op:ident, $i:expr) => {
        let plan = vec![Instruction::UnaryArithmetic {
            arithmetic: UnaryArithmetic {
                operation: UnaryOperation::$op,
                operand: Operand::Literal($i.into()),
            },
            destination: Destination::Address(Address::ZERO + 1),
        }];
        let mut mem = Memory::default();
        let ret_val = execute(&mut mem, plan, &mut None).await;
        assert_eq!(ret_val.is_err(), true);
    };
}

#[tokio::test]
async fn neg_uinteger() {
    test_unary_op!(Neg, 1u32, -1i64);
}

#[tokio::test]
async fn neg_integer() {
    test_unary_op!(Neg, 1i64, -1i64);
}

#[tokio::test]
async fn neg_a_neg_int() {
    test_unary_op!(Neg, -1i64, 1i64);
}

#[tokio::test]
async fn neg_float() {
    test_unary_op!(Neg, 1f64, -1f64);
}

#[tokio::test]
async fn neg_a_neg_float() {
    test_unary_op!(Neg, -1f64, 1f64);
}

// A neat test. We can bitflip numbers.
#[tokio::test]
async fn not_uinteger() {
    test_unary_op!(Not, 1u32, -2i64);
}

#[tokio::test]
async fn not_a_negative_one_is_zero() {
    test_unary_op!(Not, -1i64, 0i64);
}

#[tokio::test]
async fn not_float_err() {
    test_unary_op_intentional_err!(Not, 1f64);
}

#[tokio::test]
async fn abs_uinteger() {
    test_unary_op!(Abs, 1u32, 1i64);
}

#[tokio::test]
async fn abs_integer() {
    test_unary_op!(Abs, -1i64, 1i64);
}

#[tokio::test]
async fn abs_float() {
    test_unary_op!(Abs, -1f64, 1f64);
}

#[tokio::test]
async fn acos_float() {
    test_unary_op!(Acos, 1f64, 0f64);
}

// No point doing this for every single unary variant because they all
// use the same macro. The same error would appear across them all.
#[tokio::test]
async fn acos_i64_err() {
    test_unary_op_intentional_err!(Acos, 1i64);
}

#[tokio::test]
async fn acos_u32_err() {
    test_unary_op_intentional_err!(Acos, 1u32);
}

#[tokio::test]
async fn asin_float() {
    test_unary_op!(Asin, 1f64, std::f64::consts::FRAC_PI_2);
}

#[tokio::test]
async fn atan_float() {
    test_unary_op!(Atan, 1f64, std::f64::consts::FRAC_PI_4);
}

#[tokio::test]
async fn ceil_float() {
    test_unary_op!(Ceil, 1.1f64, 2.0f64);
}

#[tokio::test]
async fn cos_float() {
    test_unary_op!(Cos, std::f64::consts::PI, -1.0f64);
}

#[tokio::test]
async fn floor_float() {
    test_unary_op!(Floor, 1.1f64, 1.0f64);
}

#[tokio::test]
async fn ln_float() {
    test_unary_op!(Ln, std::f64::consts::E, 1.0f64);
}

#[tokio::test]
async fn log10_float() {
    test_unary_op!(Log10, 10f64, 1.0f64);
}

#[tokio::test]
async fn log2_float() {
    test_unary_op!(Log2, 8f64, 3.0f64);
}

#[tokio::test]
async fn sin_float() {
    test_unary_op!(Sin, std::f64::consts::PI / 2.0, 1.0f64);
}

#[tokio::test]
async fn sqrt_float() {
    test_unary_op!(Sqrt, 16f64, 4f64);
}

#[tokio::test]
async fn tan_float() {
    test_unary_op!(Tan, 1f64, 1.5574077246549023f64);
}

#[tokio::test]
async fn to_degrees_float() {
    test_unary_op!(ToDegrees, std::f64::consts::PI, 180f64);
}

#[tokio::test]
async fn to_radians_float() {
    test_unary_op!(ToRadians, 180f64, std::f64::consts::PI);
}

#[tokio::test]
async fn import_files_file_path_only() {
    let client = test_client().await;

    let mut static_data = StaticMemoryInitializer::default();
    let file_path = static_data.push(Primitive::from("cube.stl".to_owned()));
    let file_format = static_data.push(Primitive::Nil);

    // Make space for the ImportedGeometry call.
    let imported_geometry_enum_variant_header = static_data.push(Primitive::Nil);
    let imported_geometry_id = static_data.push(Primitive::Nil);
    let _imported_geometry_value_len = static_data.push(Primitive::Nil);
    let _imported_geometry_value_0 = static_data.push(Primitive::Nil);
    // Point to the beginning of the structure.
    let imported_geometry = imported_geometry_enum_variant_header;

    let img_format_addr = static_data.push(Primitive::from("Png".to_owned()));
    let output_addr = Address::ZERO + 99;
    let mut mem = static_data.finish();

    if let Err(e) = execute(
        &mut mem,
        vec![
            Instruction::ImportFiles(import_files::ImportFiles {
                files_destination: Some(Destination::StackPush),
                format_destination: None,
                arguments: vec![file_path.into(), file_format.into()],
            }),
            Instruction::ImportFiles(import_files::ImportFiles {
                files_destination: Some(Destination::StackPush),
                format_destination: Some(Destination::StackPush),
                arguments: vec![file_path.into(), file_format.into()],
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ImportFiles,
                store_response: Some(imported_geometry),
                arguments: vec![InMemory::StackPop, InMemory::StackPop],
                cmd_id: Uuid::new_v4().into(),
            }),
            Instruction::TransformImportFiles {
                source_import_files_response: InMemory::Address(imported_geometry),
                source_file_paths: InMemory::StackPop,
                destination: Destination::Address(imported_geometry),
            },
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::DefaultCameraFocusOn,
                store_response: None,
                arguments: vec![imported_geometry_id.into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::TakeSnapshot,
                store_response: Some(output_addr),
                arguments: vec![img_format_addr.into()],
                cmd_id: new_id(),
            }),
        ],
        &mut Some(client),
    )
    .await
    {
        terminate(e);
    }

    let Primitive::Bytes(b) = mem.get(&(Address::ZERO + 100)).as_ref().unwrap() else {
        panic!("wrong format in memory addr 100");
    };

    // Visually check that the image is a cube from the cube.stl file.
    use image::io::Reader as ImageReader;
    let img = ImageReader::new(std::io::Cursor::new(b))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    twenty_twenty::assert_image("tests/outputs/cube-stl.png", &img, 0.9999);
}

fn terminate(e: ExecutionFailed) {
    eprintln!("Error on instruction {}", e.instruction_index);
    eprintln!("The instruction was {:#?}", e.instruction);
    eprintln!("The error was {:#?}", e.error);
    std::process::exit(1);
}

#[tokio::test]
async fn import_files_with_file_format() {
    let client = test_client().await;

    let mut smi = StaticMemoryInitializer::default();
    let file_path = smi.push(Primitive::from("cube.stl".to_owned()));

    let file_format = smi.push(Primitive::from(ObjectHeader {
        properties: vec!["type".to_owned(), "units".to_owned(), "coords".to_owned()],
        size: 12,
    }));
    smi.push(Primitive::from("stl".to_string()));
    smi.push(Primitive::from("mm".to_string()));
    smi.push(coord::System {
        forward: coord::AxisDirectionPair {
            axis: coord::Axis::Y,
            direction: coord::Direction::Negative,
        },
        up: coord::AxisDirectionPair {
            axis: coord::Axis::Z,
            direction: coord::Direction::Positive,
        },
    });

    // Make space for the ImportedGeometry call.
    let imported_geometry_enum_variant_header = smi.push(Primitive::Nil);
    let imported_geometry_id = smi.push(Primitive::Nil);
    let _imported_geometry_value_len = smi.push(Primitive::Nil);
    let _imported_geometry_value_0 = smi.push(Primitive::Nil);
    // Point to the beginning of the structure.
    let imported_geometry = imported_geometry_enum_variant_header;

    let img_format_addr = smi.push(Primitive::from("Png".to_owned()));
    let output_addr = Address::ZERO + 99;
    let mut mem = smi.finish();

    execute(
        &mut mem,
        vec![
            Instruction::ImportFiles(import_files::ImportFiles {
                files_destination: Some(Destination::StackPush),
                format_destination: None,
                arguments: vec![file_path.into(), file_format.into()],
            }),
            Instruction::ImportFiles(import_files::ImportFiles {
                files_destination: Some(Destination::StackPush),
                format_destination: Some(Destination::StackPush),
                arguments: vec![file_path.into(), file_format.into()],
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::ImportFiles,
                store_response: Some(imported_geometry),
                arguments: vec![InMemory::StackPop, InMemory::StackPop],
                cmd_id: Uuid::new_v4().into(),
            }),
            Instruction::TransformImportFiles {
                source_import_files_response: InMemory::Address(imported_geometry),
                source_file_paths: InMemory::StackPop,
                destination: Destination::Address(imported_geometry),
            },
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::DefaultCameraFocusOn,
                store_response: None,
                arguments: vec![imported_geometry_id.into()],
                cmd_id: new_id(),
            }),
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::TakeSnapshot,
                store_response: Some(output_addr),
                arguments: vec![img_format_addr.into()],
                cmd_id: new_id(),
            }),
        ],
        &mut Some(client),
    )
    .await
    .unwrap();

    let Primitive::Bytes(b) = mem.get(&(Address::ZERO + 100)).as_ref().unwrap() else {
        panic!("wrong format in memory addr 100");
    };

    // Visually check that the image is a cube from the cube.stl file.
    use image::io::Reader as ImageReader;
    let img = ImageReader::new(std::io::Cursor::new(b))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    twenty_twenty::assert_image("tests/outputs/cube-stl.png", &img, 0.9999);
}
#[tokio::test]
async fn constants_sets_value_moves_memory_pointer() {
    let mut mem = Memory::default();

    // Create variables to hold onto the addresses.
    let pi = constants::value(&mut mem, constants::PI);
    let e = constants::value(&mut mem, constants::E);

    assert_eq!(mem.get(&pi), Some(constants::PI).as_ref());
    assert_eq!(mem.get(&e), Some(constants::E).as_ref());
}
#[tokio::test]
async fn constants_add() {
    let mut mem = Memory::default();

    // Create variables to hold onto the addresses.
    let pi = constants::value(&mut mem, constants::PI);
    let e = constants::value(&mut mem, constants::E);

    let ret_val1 = Address(mem.next_empty_cell().unwrap());
    let ret_val2 = ret_val1 + 1;

    // Compare adding two constants with two inline values
    let plan = vec![
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Add,
                operand0: Operand::Reference(pi),
                operand1: Operand::Reference(e),
            },
            destination: Destination::Address(ret_val1),
        },
        Instruction::BinaryArithmetic {
            arithmetic: BinaryArithmetic {
                operation: BinaryOperation::Add,
                operand0: Operand::Literal(constants::PI),
                operand1: Operand::Literal(constants::E),
            },
            destination: Destination::Address(ret_val2),
        },
    ];

    execute(&mut mem, plan, &mut None)
        .await
        .expect("failed to execute plan");

    assert_eq!(mem.get(&ret_val1), mem.get(&ret_val2))
}

fn new_id() -> ModelingCmdId {
    ModelingCmdId(Uuid::new_v4())
}
