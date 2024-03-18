use crate::Memory;
use kittycad_execution_plan_traits::{Address, NumericPrimitive, Primitive};

// Rust thinks this is dead-code but it will absolutely be used by consumers and
// it is in fact used in tests.

#[allow(dead_code)]
pub const E: Primitive = Primitive::NumericValue(NumericPrimitive::Float(std::f64::consts::E));

#[allow(dead_code)]
pub const PI: Primitive = Primitive::NumericValue(NumericPrimitive::Float(std::f64::consts::PI));

#[allow(dead_code)]
pub struct Constant;

impl Constant {
    #[allow(dead_code)]
    pub fn value(mem: &mut Memory, value: Primitive) -> Address {
        let mut next_address = Address(0);
        if let Some(address) = mem.next_empty_cell() {
            next_address = Address(address);
        }
        mem.set(next_address, value);
        next_address
    }
}
