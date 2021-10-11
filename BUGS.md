# *Bugs*

## Legend

| Symbol | Meaning |
| ------ | ------- |
| ❌ | Not Fixed |
| ✔️ | Fixed |
| 🔥 | Gave up |

## Catalogue

| ID | Status | Description | Expected Behaviour | Remarks |
| -- | ------ | ----------- | ------------------ | ------- |
| 0 | ✔️ | Identifiers cannot have underscores in them. | Identifiers should be allowed to have underscores anywhere (including identifiers only consisting of underscores). |
| 1 | ✔️ | 2 brackets directly adjacent (no whitespace!) to each other results in a crash (no error reported). | 2 separate tokens for each bracket should appear. |
| 2 | ❌ | Toml cannot serialise `ImmutableTokenizer`, most likely because I did not set a key for the list of tokens. | `cargo run --bin tokenizer -F toml` should not error out but write a valid file. | Rust's `toml` crate is unable to serialise enums with values inside. |
| 3 | ✔️ | In `Parser::parse_round_bracket_expression`, the tokeniser ignores the first token after the left bracket. | It should not be ignoring the first token after the left bracket. | ~~I suspect this is because the function marks the first token after the left bracket as used.~~ I had forgotten to reset the use count for the token to 0 after replacing the token in `ParserToken`. |
| 4 | ✔️ | Calling `inkwell::builder::Builder::build_int_mul` or `inkwell::builder::Builder::build_signed_int_div` on `inkwell::values::IntValue` produces a result which is equivalent to calling `inkwell::builder::Builder::build_int_sub`. | Valid multiplication/division for integer values. | I used the minus (-) sign instead of the multiply (\*) or division (/) sign in `kaleidoscope_ast::nodes::binaryop::BinaryOperatorNode::represent_expression`. See commit `7cb8a04`. |
| 5 | ✔️ | Calling `inkwell::builder::Builder::build_float_mul` or `inkwell::builder::Builder::build_float_div` produces a result equivalent to calling `inkwell::builder::Builder::build_float_sub`. | Valid multiplication/division for float values. | See bug 4. |
