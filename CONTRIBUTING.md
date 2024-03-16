# Adding a new modeling command

 - In `def_enum.rs`, under the `mod each_cmd`, add your new `struct MyNewCommand` with one field for each parameter the command has.
 - If your command responds with data:
   - In `output.rs`, add a `struct MyNewCommand` following the existing examples.
   - Then scroll to the end of the file and `impl ModelingCmdOutput for MyNewCommand {}`
   - In `ok_response.rs` add your new type to the `build_enum!` macro.
   - Derive `ModelingCmdVariant` on your struct in `def_enum.rs`.
 - Otherwise, if it doesn't respond with data:
   - Derive `ModelingCmdVariantEmpty` on your struct in `def_enum.rs`.

# Releasing a crate

In this example we'll use the modeling-cmds crate and release version 0.1.15, but you can follow
the same procedure for any of the crates in this repo, and any version.

You should **only ever bump the patch** e.g. go from 0.1.22 to 0.1.23 -- otherwise you'll need to open PRs to KittyCAD's format and engine repos to explicitly bump them to 0.2.

We do *not* consider adding a new variant to `enum ModelingCmd` to be a breaking change. So if there's a semver warning about that, ignore it.

- `git checkout -b release/modeling-cmds/0.1.15`
- Edit `modeling-cmds/Cargo.toml` and update the `version` field
- `git add --all && git commit -m "Release modeling commands 0.1.15" && git push`
- Open a PR from your branch into `main` and merge it.
- `git checkout main && git tag kittycad-modeling-cmds-0.1.15 && git push --tags`
- `cargo publish -p kittycad-modeling-cmds`
