//! Filepaths safe to use in KCL projects because they cannot escape the KCL project root.
use schemars::JsonSchema;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use typed_path::{TypedPath, UnixComponent, WindowsComponent};

/// Filepath which is guaranteed to be relative and not contain parent directory jumps like '..'
#[derive(Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, JsonSchema, Default)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SafeFilepath(String);

/// Validation error that can occur when trying to send a file to the Zoo API.
#[derive(Debug)]
pub enum PathNotSafe {
    /// Cannot use an absolute path.
    CannotBeAbsolute,
    /// Cannot use a parent path component (..)
    CannotUseParent,
}

impl std::fmt::Display for PathNotSafe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PathNotSafe::CannotBeAbsolute => "you cannot use an absolute path here",
            PathNotSafe::CannotUseParent => "you cannot use a parent jump like '..' here",
        };
        write!(f, "{}", msg)
    }
}

impl SafeFilepath {
    /// Validate if a path meets the invariants of this type,
    /// i.e. is relative and doesn't contain any parent components (..)
    pub fn validate(unparsed_user_path: &str) -> Result<Self, PathNotSafe> {
        let user_path = TypedPath::derive(unparsed_user_path);

        // Cannot be absolute.
        if user_path.is_absolute() {
            return Err(PathNotSafe::CannotBeAbsolute);
        }

        // Check all components to make sure there's no absolute or escaping from the project root.
        match user_path {
            TypedPath::Unix(path) => {
                for component in path.components() {
                    match component {
                        UnixComponent::RootDir => return Err(PathNotSafe::CannotBeAbsolute),
                        UnixComponent::ParentDir => return Err(PathNotSafe::CannotUseParent),
                        UnixComponent::CurDir | UnixComponent::Normal(..) => {}
                    }
                }
            }
            TypedPath::Windows(path) => {
                for component in path.components() {
                    match component {
                        WindowsComponent::Prefix(..) | WindowsComponent::RootDir => {
                            return Err(PathNotSafe::CannotBeAbsolute)
                        }
                        WindowsComponent::ParentDir => return Err(PathNotSafe::CannotUseParent),
                        WindowsComponent::CurDir | WindowsComponent::Normal(..) => {}
                    }
                }
            }
        }

        // All checks passed, so it's OK.
        Ok(Self(format!("{}", user_path.display())))
    }
}

/// Parsing a SafeFilepath applies the validation checks.
impl std::str::FromStr for SafeFilepath {
    type Err = PathNotSafe;

    fn from_str(unparsed_user_path: &str) -> Result<Self, Self::Err> {
        SafeFilepath::validate(unparsed_user_path)
    }
}

impl std::fmt::Display for SafeFilepath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
    struct HasPath {
        path: SafeFilepath,
    }

    #[test]
    fn test_absolute_not_allowed_unix() {
        // Absolute paths could allow reading from arbitrary files
        let input = "/foo/bar";
        let actual = SafeFilepath::validate(input);
        assert_matches!(actual, Err(PathNotSafe::CannotBeAbsolute));
    }

    #[test]
    fn test_absolute_not_allowed_windows() {
        // Test windows-style absolute paths.
        let input = r"C:\programs\bar";
        let actual = SafeFilepath::validate(input);
        assert_matches!(actual, Err(PathNotSafe::CannotBeAbsolute));
    }

    #[test]
    fn test_parent_not_allowed() {
        let input = "../../passwords/secret.txt";
        let actual = SafeFilepath::validate(input);
        assert_matches!(actual, Err(PathNotSafe::CannotUseParent));
    }

    #[test]
    fn test_success() {
        let input = "main.kcl";
        let actual = SafeFilepath::validate(input);
        assert!(actual.is_ok());
    }

    #[test]
    fn test_success_nested_file() {
        let input = "assets/bolt.step";
        let actual = SafeFilepath::validate(input);
        assert!(actual.is_ok());
    }

    #[test]
    fn test_unsafe_path_rejected_during_deserialization() {
        let input = r#"{
            "path": "../password.txt"
        }"#;
        let deserialized: Result<HasPath, _> = serde_json::from_str(input);
        assert!(deserialized.is_err());
    }

    #[test]
    fn test_unsafe_path_rejected_during_parsing() {
        let input = "../password.txt";
        assert!(input.parse::<SafeFilepath>().is_err())
    }

    #[test]
    fn test_safe_path_deserializes() {
        let input = r#"{
            "path": "file.txt"
        }"#;
        let deserialized: HasPath = serde_json::from_str(input).unwrap();
        assert_eq!(
            deserialized,
            HasPath {
                path: SafeFilepath::validate("file.txt").unwrap()
            }
        );
    }
}
