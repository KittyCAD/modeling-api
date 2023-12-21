use kittycad_execution_plan_macros::ExecutionPlanValue;
use kittycad_execution_plan_traits::{Primitive, Value};

#[derive(ExecutionPlanValue)]
struct FooConcrete {
    f: f64,
    i: usize,
    o: Option<usize>,
}

#[derive(ExecutionPlanValue)]
enum FooEnum {
    A { x: usize, y: usize },
    B { z: usize, w: usize },
    C(usize, String, f64, f32),
    D,
}

mod generics {
    use kittycad_execution_plan_macros::ExecutionPlanValue;
    use kittycad_execution_plan_traits::{Primitive, Value};

    #[derive(ExecutionPlanValue)]
    struct FooGenericWithDefault<T = f32>
    where
        Primitive: From<T>,
        T: Value,
    {
        f: f64,
        i: usize,
        t: T,
    }
}

#[test]
fn test_derive_on_enum() {
    #[derive(ExecutionPlanValue, Eq, PartialEq, Debug, Clone)]
    enum FooEnum {
        A { x: usize },
        B { y: Option<usize> },
        C(usize, String),
        D,
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
fn test_derive_on_struct() {
    #[derive(ExecutionPlanValue, PartialEq, Debug, Clone)]
    struct MyStruct {
        f: f64,
        i: usize,
        o: Option<bool>,
        s: String,
    }
    for (i, (test_name, input, expected)) in [
        (
            "Option is Some",
            MyStruct {
                f: 1.2,
                i: 2,
                o: Some(true),
                s: "hello".to_owned(),
            },
            vec![
                Primitive::from(1.2),
                Primitive::from(2u32),
                Primitive::from("Some".to_owned()),
                Primitive::from(true),
                Primitive::from("hello".to_owned()),
            ],
        ),
        (
            "Option is None",
            MyStruct {
                f: 1.2,
                i: 2,
                o: None,
                s: "hello".to_owned(),
            },
            vec![
                Primitive::from(1.2),
                Primitive::from(2u32),
                Primitive::from("None".to_owned()),
                Primitive::from("hello".to_owned()),
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
