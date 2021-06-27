# *Bugs*

## Legend

| Symbol | Meaning |
| ------ | ------- |
| ❌ | Not Fixed |
| ✔️ | Fixed |
| 🔥 | Gave up |

## Catalogue

| ID | Status | Description | Expected Behaviour |
| -- | ------ | ----------- | ------------------ |
| 0 | ✔️ | Identifiers cannot have underscores in them. | Identifiers should be allowed to have underscores anywhere (including identifiers only consisting of underscores). |
| 1 | ✔️ | 2 brackets directly adjacent (no whitespace!) to each other results in a crash (no error reported). | 2 separate tokens for each bracket should appear. |
| 2 | ❌ | Toml cannot serialise `ImmutableTokenizer`, most likely because I did not set a key for the list of tokens. | `cargo run --bin tokenizer -F toml` should not error out but write a valid file. |
