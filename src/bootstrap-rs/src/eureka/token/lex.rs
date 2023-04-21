pub fn entirely<T, F>(lex: F) -> impl Fn(&str) -> T
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    const MESSAGE: &str = "invalid value";

    move |value: &str| {
        let (token, remaining_src) = lex(value).expect(MESSAGE);

        if !remaining_src.is_empty() {
            panic!("{}", MESSAGE);
        }

        token
    }
}
