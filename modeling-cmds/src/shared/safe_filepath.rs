//! Filepaths safe to use in KCL projects because they cannot escape the KCL project root.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use typed_path::{TypedPath, UnixComponent, WindowsComponent};

/// Filepath which is guaranteed to be relative and not contain parent directory jumps like '..'
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(transparent)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct SafeFilepath(
    /// TODO: Make sure serde calls `validate` during deserialization.
    String,
);

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
        write!(
            f,
            "Paths must be relative and cannot contain parent jumps like '..' inside"
        )
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

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

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
}
