pub struct Code {
    chars: Vec<char>,
}

impl Code {
    pub fn normalize(src: &str) -> Self {
        let src = normalize(src);
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

fn normalize(src: &str) -> String {
    if src.is_empty() {
        return src.to_string();
    }

    let mut src = src.replace("\r\n", "\n");

    if src.chars().last() != Some('\n') {
        src.push('\n');
    }

    src
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut code = Code::normalize("");

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
    }

    #[test]
    fn one_line() {
        for src in ["a+b", "a+b\n", "a+b\r\n"] {
            let mut code = Code::normalize(src);

            assert_eq!(code.peek(), Some('a'));
            assert_eq!(code.pop(), Some('a'));

            assert_eq!(code.peek(), Some('+'));
            assert_eq!(code.pop(), Some('+'));

            assert_eq!(code.peek(), Some('b'));
            assert_eq!(code.pop(), Some('b'));

            assert_eq!(code.peek(), Some('\n'));
            assert_eq!(code.pop(), Some('\n'));

            assert!(code.peek().is_none());
        }
    }

    #[test]
    fn two_lines() {
        for src in [
            "A\nB",
            "A\nB\n",
            "A\nB\r\n",
            "A\r\nB",
            "A\r\nB\n",
            "A\r\nB\r\n",
        ] {
            let mut code = Code::normalize(src);

            assert_eq!(code.pop(), Some('A'));
            assert_eq!(code.pop(), Some('\n'));
            assert_eq!(code.pop(), Some('B'));
            assert_eq!(code.pop(), Some('\n'));

            assert!(code.peek().is_none());
        }
    }
}
