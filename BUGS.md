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
| 3 | ❌ | In `Parser::parse_round_bracket_expression`, the tokeniser ignores the first token after the left bracket. | It should not be ignoring the first token after the left bracket. | I suspect this is because the function marks the first token after the left bracket as used. |
