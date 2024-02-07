# Adding a new modeling command

 - In `each_cmd.rs` add your new `struct MyNewCommand` with one field for each parameter the command has.
 - In `def_enum.rs` add a new variant of `ModelingCmd` with your type, e.g. `MyNewCommand(MyNewCommand)`.
 - If your command responds with data:
   - In `output.rs`, add a `struct MyNewCommand` following the existing examples.
   - Then scroll to the end of the file and `impl ModelingCmdOutput for MyNewCommand {}`
   - In `ok_response.rs` add your new type to the `build_enum!` macro.
 - In `impl_traits.rs` follow the examples to implement `ModelingCmdVariant` for your type `MyNewCommand` using either the `impl_variant_output!` or the `impl_variant_empty!` macro. If your command responds with data, use the former. If your command has no response, use the latter.

# Releasing a crate

In this example we'll use the modeling-cmds crate and release version 0.1.15, but you can follow
the same procedure for any of the crates in this repo, and any version.

- `git checkout -b release/modeling-cmds/0.1.15`
- Edit `modeling-cmds/Cargo.toml` and update the `version` field
- `git add --all && git commit -m "Release modeling commands 0.1.15" && git push`
- Open a PR from your branch into `main` and merge it.
- `git checkout main && git tag kittycad-modeling-cmds-0.1.15 && git push --tags`
- `cargo publish -p kittycad-modeling-cmds`
