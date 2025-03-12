use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A wrapper for chrono types, since we need to impl Value for them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default, Deserialize, Serialize, JsonSchema)]
pub struct DateTimeLocal {
    value: chrono::DateTime<chrono::Local>,
}

impl From<DateTimeLocal> for chrono::DateTime<chrono::Local> {
    fn from(value: DateTimeLocal) -> Self {
        value.value
    }
}

impl From<chrono::DateTime<chrono::Local>> for DateTimeLocal {
    fn from(value: chrono::DateTime<chrono::Local>) -> Self {
        Self { value }
    }
}
