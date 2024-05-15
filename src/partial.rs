use std::fmt::Debug;

use openapiv3::SchemaReference;

pub struct PartialTypeScript {
    pub ts: String,
    pub references: Vec<SchemaReference>,
}
