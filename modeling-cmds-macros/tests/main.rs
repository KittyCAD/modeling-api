use kittycad_execution_plan_macros::{ExecutionPlanFromMemory, ExecutionPlanValue};
use kittycad_execution_plan_traits::{Primitive, Value};

#[test]
fn test_derive_value_on_enum() {
    #[derive(ExecutionPlanValue, Eq, PartialEq, Debug, Clone)]
    enum FooEnum {
        A { x: usize },
        B { y: Option<usize> },
        C(usize, String),
        D,
        E(Box<usize>),
        F { boxed: Box<usize> },
    }
    for (i, (test_name, input, expected)) in [
        (
            "named fields",
            FooEnum::A { x: 3 },
            vec![Primitive::from("A".to_owned()), Primitive::from(3u32)],
        ),
        (
            "named fields with Option::Some",
            FooEnum::B { y: Some(3) },
            vec![
                Primitive::from("B".to_owned()),
                Primitive::from("Some".to_owned()),
                Primitive::from(3u32),
            ],
        ),
        (
            "named fields with Option::None",
            FooEnum::B { y: None },
            vec![Primitive::from("B".to_owned()), Primitive::from("None".to_owned())],
        ),
        (
            "positional fields",
            FooEnum::C(4, "hello".to_owned()),
            vec![
                Primitive::from("C".to_owned()),
                Primitive::from(4u32),
                Primitive::from("hello".to_owned()),
            ],
        ),
        ("unit variant", FooEnum::D, vec![Primitive::from("D".to_owned())]),
        (
            "Boxed unnamed field",
            FooEnum::E(Box::new(2)),
            vec![Primitive::from("E".to_owned()), Primitive::from(2usize)],
        ),
        (
            "Boxed unnamed field",
            FooEnum::F { boxed: Box::new(2) },
            vec![Primitive::from("F".to_owned()), Primitive::from(2usize)],
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let actual = input.clone().into_parts();
        assert_eq!(
            actual, expected,
            "failed test {i}, '{test_name}'.\nActual parts (left) != expected parts (right)"
        );
        let mut actual_iter = actual.into_iter().map(Some);
        let inverted = FooEnum::from_parts(&mut actual_iter).expect("from_parts should succeed");
        assert_eq!(inverted, input, "failed test {i}, '{test_name}'.\nInput value (right) != input value turned into parts then back into value (left).");
    }
}

#[test]
fn test_derive_value_on_struct() {
    #[derive(ExecutionPlanValue, PartialEq, Debug, Clone)]
    struct MyStruct {
        f: f64,
        i: usize,
        o: Option<bool>,
        s: String,
        b: Box<usize>,
    }
    for (i, (test_name, input, expected)) in [
        (
            "Option is Some",
            MyStruct {
                f: 1.2,
                i: 2,
                o: Some(true),
                s: "hello".to_owned(),
                b: Box::new(2),
            },
            vec![
                Primitive::from(1.2),
                Primitive::from(2u32),
                Primitive::from("Some".to_owned()),
                Primitive::from(true),
                Primitive::from("hello".to_owned()),
                Primitive::from(2usize),
            ],
        ),
        (
            "Option is None",
            MyStruct {
                f: 1.2,
                i: 2,
                o: None,
                s: "hello".to_owned(),
                b: Box::new(2),
            },
            vec![
                Primitive::from(1.2),
                Primitive::from(2u32),
                Primitive::from("None".to_owned()),
                Primitive::from("hello".to_owned()),
                Primitive::from(2usize),
            ],
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let actual = input.clone().into_parts();
        assert_eq!(
            actual, expected,
            "failed test {i}, '{test_name}'.\nActual parts (left) != expected parts (right)"
        );
        let mut actual_iter = actual.into_iter().map(Some);
        let inverted = MyStruct::from_parts(&mut actual_iter).expect("from_parts should succeed");
        assert_eq!(inverted, input, "failed test {i}, '{test_name}'.\nInput value (right) != input value turned into parts then back into value (left).");
    }
}

#[test]
fn test_derive_from_memory_on_struct() {
    #[derive(Debug, Clone, ExecutionPlanFromMemory)]
    pub struct Extrude {
        pub target: u32,
        pub distance: f64,
        pub cap: bool,
    }
}
