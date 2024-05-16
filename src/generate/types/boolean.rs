use std::borrow::Cow;

pub fn generate_boolean<'a>() -> Cow<'a, str> {
    Cow::Borrowed("boolean")
}
