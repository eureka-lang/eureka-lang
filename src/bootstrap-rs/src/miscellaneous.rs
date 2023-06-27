pub fn missing(something: &str) -> String {
    format!("missing {something}")
}

pub trait DisplayName {
    fn display_name() -> &'static str;
}
