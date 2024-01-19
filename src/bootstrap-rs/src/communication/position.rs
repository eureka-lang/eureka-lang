pub use restricted::Position;

mod restricted {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Position {
        line: usize,
        column: usize,
    }

    impl Position {
        pub fn new(line: usize, column: usize) -> Position {
            assert!(line >= 1 && column >= 1);
            Position { line, column }
        }

        pub fn line(&self) -> usize {
            self.line
        }

        pub fn column(&self) -> usize {
            self.column
        }
    }
}

impl Position {
    pub fn start() -> Position {
        Position::new(1, 1)
    }

    pub fn set_line(&mut self, line: usize) {
        *self = Position::new(line, self.column());
    }

    pub fn set_column(&mut self, column: usize) {
        *self = Position::new(self.line(), column);
    }

    pub fn advance(&mut self, c: char) {
        match c {
            ' '..='~' => self.set_column(self.column() + 1),
            '\n' => {
                self.set_line(self.line() + 1);
                self.set_column(1);
            }
            _ => unimplemented!(),
        }
    }

    pub fn advance_str(&mut self, s: &str) {
        for c in s.chars() {
            self.advance(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let position = Position::new(1, 2);

        assert_eq!(position.line(), 1);
        assert_eq!(position.column(), 2);
    }

    #[test]
    fn mut_position() {
        let mut position = Position::new(2, 9);

        assert_eq!(position.line(), 2);
        assert_eq!(position.column(), 9);

        position.set_line(5);

        assert_eq!(position.line(), 5);
        assert_eq!(position.column(), 9);

        position.set_column(4);

        assert_eq!(position.line(), 5);
        assert_eq!(position.column(), 4);
    }

    #[test]
    fn advance_position() {
        let mut position = Position::start();

        assert_eq!((position.line(), position.column()), (1, 1));
        position.advance(' ');
        assert_eq!((position.line(), position.column()), (1, 2));
        position.advance('!');
        assert_eq!((position.line(), position.column()), (1, 3));
        position.advance('9');
        assert_eq!((position.line(), position.column()), (1, 4));
        position.advance('\n');
        assert_eq!((position.line(), position.column()), (2, 1));
        position.advance('~');
        assert_eq!((position.line(), position.column()), (2, 2));
        position.advance('a');
        assert_eq!((position.line(), position.column()), (2, 3));
        position.advance('\n');
        assert_eq!((position.line(), position.column()), (3, 1));
        position.advance('A');
        assert_eq!((position.line(), position.column()), (3, 2));
        position.advance('\n');
        assert_eq!((position.line(), position.column()), (4, 1));
        position.advance('\n');
        assert_eq!((position.line(), position.column()), (5, 1));
    }
}
