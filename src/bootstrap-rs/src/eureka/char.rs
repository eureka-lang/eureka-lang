use crate::communication::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Char {
    value: char,
}

impl TryFrom<char> for Char {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if (' ' <= c && c <= '~') || c == '\n' {
            Ok(Char { value: c })
        } else {
            Err(Error::UnexpectedChar(c))
        }
    }
}

impl From<Char> for char {
    fn from(c: Char) -> Self {
        c.value
    }
}
