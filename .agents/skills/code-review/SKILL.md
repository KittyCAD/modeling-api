---
name: kcmc-code-review
description: "Review PRs to kittycad-modeling-cmds like Adam Chalmers would"
license: MIT
---
Things to consider when reviewing these PRs.

## Backwards compatibility
Be very careful about backwards compatibility. The Zoo engine might update and pull in the kittycad-modeling-cmds crate before clients (e.g. the API or KCL runtime). Or it could be the other way around! You CANNOT guarantee that they'll both get the new modeling commands definition at the same time.

These kinds of changes are safe and backwards-compatible:

 - Adding a new modeling command
 - Adding a new field to an old modeling command (request or response), as long as that field has `#[serde(default)]` to handle if the client/server doesn't send that new field. If it uses `#[serde(default)]`, consider also adding `#[builder(default)]` so that users of this library have fewer breaking Rust changes. Sometimes you want to force people to think about sending the new field though, so the builder default is not necessary
 - Making an old field optional. We frequently do this to deprecate an old field and suggest users use a new field instead. 

These kinds of changes can break production:

 - Adding a new mandatory field (i.e. with no `#[serde(default)]`). If an old client doesn't send that field, then the server will get an error trying to deserialize those old requests. 
 - Removing a modeling command

Users should not remove fields/commands unless they know all clients (KCL runtime, the API, the Synera node, the CLI) have been updated and no longer send that field/command.

## Secondary concerns

 - Does the documentation properly explain the point of each field and command?
 - Is it clear what kind of ID is expected for each UUID field? E.g. the ID of an edge, a solid, etc
 - Don't bother testing that `serde` or `bun` crates work properly. If you want to write a unit test for asserting that JSON sent by old clients is still accepted by new servers (or vice-versa) that's fine. But don't write tests that basically check that a field marked with `#[serde(default)]` is actually optional. You can trust serde.
