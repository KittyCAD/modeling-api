use kittycad_execution_plan_macros::ExecutionPlanValue;

#[derive(ExecutionPlanValue)]
struct FooConcrete {
    f: f64,
    i: usize,
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
