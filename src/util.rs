/// Creates a quoted value (e.g. `"example"`).
pub fn quote<S>(s: S) -> Quote<S> {
    Quote(s)
}

/// A quoted value (e.g. `"example"`).
pub struct Quote<S>(S);

impl<S> fmt::Display for Quote<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

use std::fmt;
