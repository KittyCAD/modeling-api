use kittycad_execution_plan_macros::MyMacro;

#[derive(MyMacro)]
struct Foo {
    f: f64,
    i: usize,
}

#[test]
fn check_impl() {}
