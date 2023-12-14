# Adding a new modeling command

 - In `each_command.rs` add your new `struct MyNewCommand`
 - In `def_enum.rs` add a new variant of `ModelingCmd` with your type, e.g. `MyNewCommand(MyNewCommand)`.
 - If your command responds with data:
   - In `output.rs`, add a `struct MyNewCommand` following the existing examples.
   - Then scroll to the end of the file and `impl<'de> ModelingCmdOutput<'de> for MyNewCommand {}`
 - In `impl_traits.rs` follow the examples to implement `ModelingCmdVariant<'de>` for your type `MyNewCommand` using either the `impl_variant_output!` or the `impl_variant_empty!` macro. If your command responds with data, use the former. If your command has no response, use the latter.