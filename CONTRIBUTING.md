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

This repo uses a `justfile`. Be sure to [install `just`](https://github.com/casey/just?tab=readme-ov-file#packages) if you haven't already.

 - Create a release branch: `just start-release modeling-cmds`.
 - Open a PR (hint: `just start-release` output should include a link to GitHub which will open a release PR).
 - Merge the PR
 - Run `just finish-release modeling-cmds`.

The `just` scripts above accept any workspace member as their first argument. For example, you could replace `modeling-cmds` with `modeling-cmds-macros` there.

## Note on semver

The `just` scripts also accept a second arg, which defaults to `patch` -- this is the kind of semver bump to make. Technically you can specify `minor` or `major` too, but you should **almost always just bump the patch** e.g. go from 0.1.22 to 0.1.23. We don't really care about semver accuracy as Zoo engineers are the only people using this crate currently. Once other users need these crates, we'll start enforcing semver -- until then, convenience is really what matters. If you ever bump `modeling-cmds` major/minor versions, you'll need to open PRs to KittyCAD's format and engine repos to explicitly bump them to 0.2. Talk to Adam Chalmers before bumping the minor or major version.
