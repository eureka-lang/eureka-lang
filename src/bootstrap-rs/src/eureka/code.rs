pub struct Code {
    chars: Vec<char>,
}

impl Code {
    pub fn new(src: &str) -> Self {
        let mut chars: Vec<char> = src.chars().collect();
        chars.reverse();
        Self { chars }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.last().copied()
    }

    pub fn pop(&mut self) -> Option<char> {
        self.chars.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut code = Code::new("");

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
    }

    #[test]
    fn non_empty() {
        let mut code = Code::new("a+b");

        assert_eq!(code.peek(), Some('a'));
        assert_eq!(code.pop(), Some('a'));

        assert_eq!(code.peek(), Some('+'));
        assert_eq!(code.pop(), Some('+'));

        assert_eq!(code.peek(), Some('b'));
        assert_eq!(code.pop(), Some('b'));

        assert!(code.peek().is_none());
    }
}
