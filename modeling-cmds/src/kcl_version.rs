use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// Which KCL versions does Zoo support?
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub enum KclVersion {
    /// Original KCL released in 2025
    #[default]
    V1,
    /// KCL v2 is the same as KCL v1, except
    /// that it supports the `region` function.
    V2,
    /// KCL v3 is currently in development.
    V3Preview,
    // When you add a new version, please add it to the error string in KclVersionError's
    // Display and FromStr impls.
}

impl KclVersion {
    /// Get the canonical string representation for each version.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
            Self::V2 => "v2",
            Self::V3Preview => "v3_preview",
        }
    }
}

#[derive(Debug)]
pub struct KclVersionError;

impl core::error::Error for KclVersionError {}

impl std::fmt::Display for KclVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unrecognized version. Valid versions are v1, v2 and (experimentally) v3_preview"
        )
    }
}

impl FromStr for KclVersion {
    type Err = KclVersionError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            "v3_preview" => Ok(Self::V3Preview),
            _other => Err(KclVersionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_str() {
        for input in [KclVersion::V1, KclVersion::V2, KclVersion::V3Preview] {
            let serialized = input.as_str();
            let deserialized: KclVersion = serialized.parse().unwrap();
            assert_eq!(input, deserialized);
        }
    }
}
