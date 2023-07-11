use crate::communication::INVALID_VALUE;

pub fn entirely<T, F>(lex: F) -> impl Fn(&str) -> T
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    move |value: &str| {
        let (token, remaining_src) = lex(value).expect(INVALID_VALUE);

        if !remaining_src.is_empty() {
            panic!("{}", INVALID_VALUE);
        }

        token
    }
}
