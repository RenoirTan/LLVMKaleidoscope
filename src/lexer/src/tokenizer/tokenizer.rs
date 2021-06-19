use super::Stream;

/// The tokeniser which iterates over the characters in a file stream and
/// yields a stream of tokens.
pub struct Tokenizer {
    pub stream: Stream
}
