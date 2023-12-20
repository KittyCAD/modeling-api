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

#[derive(ExecutionPlanValue)]
struct FooConcrete {
    f: f64,
    i: usize,
}
