use crate::{
    context::Context, processor::Processor, segment::Segment, tokenizer::Tokenizer, Error, Pattern,
};
use std::collections::HashMap;

/// The layout you describe the tag
pub struct Layout<E: 'static> {
    pub patterns: HashMap<String, Pattern<E>>,
}

impl<E: std::fmt::Display> Layout<E> {
    pub fn new() -> Self {
        Self {
            patterns: Default::default(),
        }
    }

    pub fn proccess(&self, s: &str) -> Result<String, Error<E>> {
        Processor::new(self, false).process(&mut Tokenizer::new(s.chars()))
    }

    pub fn construct(
        &self,
        tag_name: impl Into<String>,
        args: HashMap<String, Option<String>>,
        inner: impl Into<String>,
    ) -> Result<String, Error<E>> {
        let tag_name = tag_name.into();
        let null_str = Ok(Default::default());

        match self.patterns.get(&tag_name) {
            None => null_str,

            Some(pattern) => {
                let mut context = Context::new(tag_name, args, inner);
                let mut result = String::new();

                for segment in &pattern.segments {
                    let empty = String::from("");
                    match segment {
                        Segment::Static(s) => result.push_str(s),
                        Segment::Owned(s) => result.push_str(s),
                        Segment::Linked(key) => result.push_str(
                            match match context.args.get(key) {
                                Some(v) => v,
                                None => return null_str,
                            } {
                                Some(v) => v,
                                None => return null_str,
                            },
                        ),
                        Segment::Computed(func) => result.push_str(&func(&mut context)?),
                        Inner => result.push_str(&context.inner),
                    }
                }

                Ok(result)
            }
        }
    }

    pub fn pattern(mut self, tag: impl Into<String>, e: Pattern<E>) -> Result<Self, Error<E>> {
        if let Some(_) = self.patterns.insert(tag.into(), e) {
            Err(Error::PatternAlreadyRegistered)
        } else {
            Ok(self)
        }
    }
}
