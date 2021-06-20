/// An enumerator of possible keywords that can be encountered in Kaleidoscope.
#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    /// `def` keyword. Define a function.
    Def,
    /// `extern` keyword. For foreign function interfaces.
    Extern,
}

use kaleidoscope_macro::impl_display;
impl_display!(Keyword);
