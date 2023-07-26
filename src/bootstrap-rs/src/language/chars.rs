use crate::communication::Error;
pub use restricted::Chars;

mod restricted {
    use crate::communication::{Error, Position, PositionError};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Chars<C: TryFrom<char, Error = Error> + Into<char> + Copy> {
        values: Vec<C>,
        position: Position,
    }

    impl<C: TryFrom<char, Error = Error> + Into<char> + Copy> Chars<C> {
        pub fn try_new(src: &str) -> Result<Self, PositionError> {
            let mut values: Vec<C> = Vec::with_capacity(src.len());
            let mut position = Position::start();

            for c in src.chars() {
                match C::try_from(c) {
                    Ok(c) => {
                        position.advance(c.into());
                        values.push(c);
                    }
                    Err(e) => return Err(PositionError::new(position, e)),
                }
            }

            values.reverse();

            Ok(Chars {
                values,
                position: Position::start(),
            })
        }

        pub fn peek(&self) -> Option<char> {
            self.values.last().copied().map(|c| c.into())
        }

        pub fn pop(&mut self) -> Option<char> {
            if let Some(c) = self.values.pop() {
                self.position.advance(c.into());
                Some(c.into())
            } else {
                None
            }
        }

        pub fn position(&self) -> Position {
            self.position
        }
    }
}

impl<C: TryFrom<char, Error = Error> + Into<char> + Copy> Chars<C> {
    pub fn new(src: &str) -> Self {
        Chars::try_new(src).unwrap()
    }

    pub fn try_take(&mut self, predicate: impl Fn(char) -> bool, buffer: &mut String) -> bool {
        self.take(predicate, buffer).is_ok()
    }

    pub fn take_while(&mut self, predicate: impl Fn(char) -> bool, buffer: &mut String) {
        while self.try_take(&predicate, buffer) {}
    }

    pub fn take(
        &mut self,
        predicate: impl Fn(char) -> bool,
        buffer: &mut String,
    ) -> Result<(), Error> {
        if let Some(c) = self.peek() {
            if predicate(c) {
                buffer.push(self.pop().unwrap());
                Ok(())
            } else {
                Err(Error::UnexpectedChar(c))
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }
}
