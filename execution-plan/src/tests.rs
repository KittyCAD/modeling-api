use std::env;

use kittycad_modeling_cmds::shared::Point3d;
use kittycad_modeling_session::{Session, SessionBuilder};
use uuid::Uuid;

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
    assert_eq!(mem.0[0], Some(2.0.into()));
    assert_eq!(mem.0[1], Some(3.0.into()));
    assert_eq!(mem.0[2], Some(4.0.into()));

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
async fn api_call_no_output() {
    let mut mem = Memory::default();
    let client = test_client().await;

    const CUBE_WIDTH: f64 = 10.0;

    // Choose a path ID, map it to a memory address.
    let path = new_id();
    let path_id_addr = Address(0);

    let start = Point3d {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -CUBE_WIDTH,
    };
    let start_addr = Address(1);

    mem.set(path_id_addr, Primitive::from(path.0));
    mem.set_composite(start_addr, start);

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
            // Extend the path.
            Instruction::ApiRequest(ApiRequest {
                endpoint: Endpoint::MovePathPen,
                store_response: None,
                arguments: vec![path_id_addr, start_addr],
                cmd_id: new_id(),
            }),
        ],
        client,
    )
    .await
    .unwrap();

    dbg!(&mem.0[..10]);
}

fn new_id() -> ModelingCmdId {
    ModelingCmdId(Uuid::new_v4())
}
