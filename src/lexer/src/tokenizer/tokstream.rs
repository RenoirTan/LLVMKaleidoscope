use super::{FileStream, Tokenizer};

#[derive(Copy, Clone)]
pub struct LexerTupleRef<'a>(pub &'a FileStream, pub &'a Tokenizer);

pub struct LexerTupleMut<'a>(pub &'a mut FileStream, pub &'a mut Tokenizer);
