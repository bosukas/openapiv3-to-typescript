use std::borrow::Cow;

use openapiv3::IntegerType;

pub fn generate_integer(_: &IntegerType) -> Cow<str> {
    Cow::Borrowed("number")
}
