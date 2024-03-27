//! Bumps versions in Cargo.toml.
use anyhow::Context;
use clap::Parser;
use toml_edit::{value, DocumentMut};

fn main() {
    let args = Args::parse();
    if let Err(e) = inner_main(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn inner_main(args: Args) -> anyhow::Result<()> {
    let cargo_dot_toml = std::fs::read(&args.manifest_path).context("Could not read chosen Cargo.toml")?;
    let cargo_dot_toml = String::from_utf8(cargo_dot_toml).context("Invalid UTF-8 in chosen Cargo.toml")?;
    let mut doc = cargo_dot_toml
        .parse::<DocumentMut>()
        .context("Invalid TOML in Cargo.toml")?;
    update_semver(args.bump, &mut doc).context("Could not bump semver")?;
    std::fs::write(args.manifest_path, doc.to_string()).context("Could not write updated Cargo.toml")?;
    Ok(())
}

fn parse_version(cargo_dot_toml: &mut DocumentMut) -> anyhow::Result<semver::Version> {
    let current_version = cargo_dot_toml["package"]["version"]
        .to_string()
        // Clean quotations and whitespace.
        .replace([' ', '"'], "");

    semver::Version::parse(&current_version).context("Could not parse semver version")
}

/// Update the given TOML document (for a Cargo.toml file) by bumping its `version` field.
/// What kind of bump (major, minor, patch) is given by the `bump` argument.
fn update_semver(bump: Option<SemverBump>, cargo_dot_toml: &mut DocumentMut) -> anyhow::Result<()> {
    let current_version = parse_version(cargo_dot_toml)?;

    // Get the next version.
    let Some(bump) = bump else {
        println!("{current_version}");
        return Ok(());
    };
    let mut next_version = current_version;
    match bump {
        SemverBump::Major => next_version.major += 1,
        SemverBump::Minor => next_version.minor += 1,
        SemverBump::Patch => next_version.patch += 1,
    };

    // Update the Cargo.toml
    cargo_dot_toml["package"]["version"] = value(next_version.to_string());
    println!("{next_version}");
    Ok(())
}

/// Bumps versions in Cargo.toml
#[derive(Parser, Debug)]
struct Args {
    /// Which Cargo.toml file to bump.
    #[arg(short, long)]
    manifest_path: String,

    /// What part of the semantic version (major, minor or patch) to bump.
    /// If not given, bumper will just print the current version and then exit.
    #[arg(short, long)]
    bump: Option<SemverBump>,
}

#[derive(Debug, Clone, Copy)]
enum SemverBump {
    Major,
    Minor,
    Patch,
}

impl std::str::FromStr for SemverBump {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "patch" => Ok(Self::Patch),
            _ => Err("valid options are 'major', 'minor' and 'patch'.".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
[package]
name = "bumper"
version = "0.1.0"

[dependencies]
anyhow = "1.0.81"
        "#;

    #[test]
    fn test_bump_minor() {
        let mut cargo_dot_toml = EXAMPLE.parse::<DocumentMut>().unwrap();
        update_semver(Some(SemverBump::Minor), &mut cargo_dot_toml).unwrap();
        assert_eq!(
            cargo_dot_toml.to_string(),
            r#"
[package]
name = "bumper"
version = "0.2.0"

[dependencies]
anyhow = "1.0.81"
        "#
        );
    }

    #[test]
    fn test_bump_major() {
        let mut cargo_dot_toml = EXAMPLE.parse::<DocumentMut>().unwrap();
        update_semver(Some(SemverBump::Major), &mut cargo_dot_toml).unwrap();
        assert_eq!(
            cargo_dot_toml.to_string(),
            r#"
[package]
name = "bumper"
version = "1.1.0"

[dependencies]
anyhow = "1.0.81"
        "#
        );
    }

    #[test]
    fn test_bump_patch() {
        let mut cargo_dot_toml = EXAMPLE.parse::<DocumentMut>().unwrap();
        update_semver(Some(SemverBump::Patch), &mut cargo_dot_toml).unwrap();
        assert_eq!(
            cargo_dot_toml.to_string(),
            r#"
[package]
name = "bumper"
version = "0.1.1"

[dependencies]
anyhow = "1.0.81"
        "#
        );
    }
}
