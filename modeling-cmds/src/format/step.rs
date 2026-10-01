use bon::Builder;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::coord;

/// After importing, how should this model's data be represented?
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "StepImportTargetRepresentation", rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass_enum,
    pyo3::pyclass(name = "StepImportTargetRepresentation", from_py_object)
)]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum TargetRepresentation {
    /// Mesh of 2D geometry
    Mesh,
    /// Boundary representation
    Brep,
}

const DEFAULT_REPR: TargetRepresentation = TargetRepresentation::Brep;

/// Import models in STEP format.
pub mod import {
    use super::*;

    /// Options for importing STEP format.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(default, rename = "StepImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "StepImportOptions", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Co-ordinate system of input data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        #[builder(default = *coord::KITTYCAD)]
        pub coords: coord::System,

        /// Splits all closed faces into two open faces.
        ///
        /// Defaults to `false` but is implicitly `true` when importing into the engine.
        #[builder(default)]
        pub split_closed_faces: bool,

        /// What representation should be used for this file after it's imported?
        #[builder(default = DEFAULT_REPR)]
        pub target_representation: TargetRepresentation,
    }

    #[cfg(feature = "python")]
    #[pyo3_stub_gen::derive::gen_stub_pymethods]
    #[pyo3::pymethods]
    impl Options {
        #[new]
        /// Set the options to their defaults.
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                target_representation: DEFAULT_REPR,
                coords: *coord::KITTYCAD,
                split_closed_faces: false,
            }
        }
    }
}

/// Export models in STEP format.
pub mod export {
    use super::*;
    use crate::units::UnitLength;

    /// Specifies the STEP application protocol schema for export.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "StepSchema", rename_all = "snake_case")]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "StepSchema", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Schema {
        /// AP203 edition 2.
        Ap203,

        /// AP214.
        Ap214,

        /// AP242.
        ///
        /// This is the default setting.
        #[default]
        Ap242,
    }

    /// Describes the presentation style of the EXPRESS exchange format.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "StepPresentation", rename_all = "snake_case")]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "StepPresentation", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Presentation {
        /// Condenses the text to reduce the size of the file.
        Compact,

        /// Add extra spaces to make the text more easily readable.
        ///
        /// This is the default setting.
        #[default]
        Pretty,
    }

    /// Options for exporting STEP format.
    #[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize, Builder)]
    #[serde(default, rename = "StepExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "StepExportOptions", from_py_object)
    )]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        #[builder(default = default_coords())]
        pub coords: coord::System,

        /// Timestamp override.
        pub created: Option<chrono::DateTime<chrono::Utc>>,

        /// Export length unit.
        ///
        /// Defaults to meters.
        #[builder(default = default_units())]
        pub units: UnitLength,

        /// Presentation style.
        #[builder(default = default_presentation())]
        pub presentation: Presentation,

        /// STEP application protocol schema. Defaults to AP242.
        #[builder(default)]
        pub schema: Schema,
    }

    #[cfg(feature = "python")]
    #[pyo3_stub_gen::derive::gen_stub_pymethods]
    #[pyo3::pymethods]
    impl Options {
        #[new]
        /// Set the options to their defaults.
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: default_coords(),
                created: None,
                units: default_units(),
                presentation: default_presentation(),
                schema: Schema::default(),
            }
        }
    }

    const fn default_presentation() -> Presentation {
        Presentation::Pretty
    }

    const fn default_coords() -> coord::System {
        *coord::KITTYCAD
    }

    const fn default_units() -> UnitLength {
        UnitLength::Meters
    }
}

#[cfg(test)]
mod tests {
    use super::export::{Options, Presentation, Schema};
    use crate::{coord, format::OutputFormat3d, shared::FileExportFormat, units::UnitLength};

    #[test]
    fn schema_defaults_to_ap242() {
        assert_eq!(Schema::default(), Schema::Ap242);
        assert_eq!(Options::default().schema, Schema::Ap242);
        assert_eq!(Options::builder().build(), Options::default());
        let expected = OutputFormat3d::Step(Options::default());
        assert_eq!(OutputFormat3d::from(FileExportFormat::Step), expected);
        assert_eq!(
            serde_json::from_str::<OutputFormat3d>(r#"{"type":"step"}"#).unwrap(),
            expected
        );
    }

    #[test]
    fn existing_step_options_default_to_ap242() {
        let json = serde_json::json!({
            "type": "step",
            "coords": *coord::OPENGL,
            "created": "2026-10-01T12:00:00Z",
            "units": "mm",
            "presentation": "compact"
        });
        let options = Options::builder()
            .coords(*coord::OPENGL)
            .created("2026-10-01T12:00:00Z".parse().unwrap())
            .units(UnitLength::Millimeters)
            .presentation(Presentation::Compact)
            .build();
        assert_eq!(
            serde_json::from_value::<OutputFormat3d>(json).unwrap(),
            OutputFormat3d::Step(options)
        );
    }

    #[test]
    fn schema_option_round_trips() {
        for (name, schema) in [
            ("ap203", Schema::Ap203),
            ("ap214", Schema::Ap214),
            ("ap242", Schema::Ap242),
        ] {
            let json = serde_json::json!({
                "type": "step",
                "coords": *coord::OPENGL,
                "created": "2026-10-01T12:00:00Z",
                "units": "mm",
                "presentation": "compact",
                "schema": name
            });
            let expected = OutputFormat3d::Step(
                Options::builder()
                    .coords(*coord::OPENGL)
                    .created("2026-10-01T12:00:00Z".parse().unwrap())
                    .units(UnitLength::Millimeters)
                    .presentation(Presentation::Compact)
                    .schema(schema)
                    .build(),
            );
            let actual: OutputFormat3d = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(serde_json::to_value(actual).unwrap(), json);
        }
    }

    #[test]
    fn rejects_unknown_schema() {
        let error = serde_json::from_str::<OutputFormat3d>(r#"{"type":"step","schema":"ap999"}"#).unwrap_err();
        assert!(error.to_string().contains("unknown variant `ap999`"));
    }
}
