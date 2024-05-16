use std::borrow::Cow;

use openapiv3::NumberType;

pub fn generate_number(_: &NumberType) -> Cow<str> {
    Cow::Borrowed("number")
}
