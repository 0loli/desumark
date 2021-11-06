use std::collections::HashMap;

/// General context structure which describes the way we process the text
pub struct Context {
    pub tag: String,
    pub args: HashMap<String, Option<String>>,
    pub inner: String,
}

impl Context {
    pub fn new(
        tag: impl Into<String>,
        args: HashMap<String, Option<String>>,
        inner: impl Into<String>,
    ) -> Self {
        Self {
            tag: tag.into(),
            inner: inner.into(),
            args,
        }
    }
}
