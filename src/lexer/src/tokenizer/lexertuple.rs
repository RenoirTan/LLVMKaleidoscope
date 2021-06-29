//! Some tuples storing a [`FileStream`] and a [`Tokenizer`].
//! 
//! These tuples are merely for simplifying the transport of these 2
//! objects, which must come together in order to create tokens.

use super::{FileStream, Tokenizer};

/// A tuple storing immutable references to a [`FileStream`] and a
/// [`Tokenizer`]. This is basically useless because you need to be able to
/// mutate both the stream and the tokenizer to create tokens. In order to
/// use that functionality, please see [`LexerTupleMut`].
#[derive(Copy, Clone)]
pub struct LexerTupleRef<'a>(pub &'a FileStream, pub &'a Tokenizer);

/// A tuple storing mutable references to a [`FileStream`] and a [`Tokenizer`].
/// Both of these objects can be used to create a stream of tokens.
pub struct LexerTupleMut<'a>(pub &'a mut FileStream, pub &'a mut Tokenizer);

/// A macro that can destructure or create a new [`LexerTupleRef`] object
/// cheaply.
#[macro_export]
macro_rules! ltupleref {
    ($stream: ident, $tokenizer: ident) => {
        $crate::tokenizer::LexerTupleRef($stream, $tokenizer)
    };
    ($stream: expr, $tokenizer: expr) => {
        $crate::tokenizer::LexerTupleRef($stream, $tokenizer)
    };
}

/// A macro that can destructure or create a new [`LexerTupleMut`] object
/// cheaply.
#[macro_export]
macro_rules! ltuplemut {
    ($stream: ident, $tokenizer: ident) => {
        $crate::tokenizer::LexerTupleMut($stream, $tokenizer)
    };
    ($stream: expr, $tokenizer: expr) => {
        $crate::tokenizer::LexerTupleMut($stream, $tokenizer)
    };
}
