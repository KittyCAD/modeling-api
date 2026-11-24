# KittyCAD's Modeling API

This defines the API KittyCAD types and contracts. Use this to know the body and return value of APIs in Rust.

# Modeling API

Check out the documentation below. The contracts define these websocket endpoints.

- [API documentation](https://zoo.dev/docs/developer-tools/api/modeling)
- [Opens a WebSocket to a Zoo KittyCAD engine instance](https://zoo.dev/docs/developer-tools/api/modeling/opens-a-websocket-to-a-zoo-kittycad-engine-instance)

# If you are looking for the API clients

- [kittycad.rs (Rust client)](https://github.com/KittyCAD/kittycad.rs)
- [kittycad.ts (Typescript client)](https://github.com/KittyCAD/kittycad.ts)

# Beware

This project does not use semver. We are using 0.1.x for everything. If you use this crate, commit your Cargo.lock to avoid being broken when we publish a new version. 

Why? Because we use this primarily for KittyCAD server and clients, where we are on top of all changes.

# Published rust crate

We publish the rust crate as `kittycad-modeling-cmds`

 - Crate page [kittycad-modeling-cmds](https://crates.io/crates/kittycad-modeling-cmds)
 - Check out the docs [kittycad-modeling-cmds](https://docs.rs/kittycad-modeling-cmds/latest/kittycad_modeling_cmds/)
