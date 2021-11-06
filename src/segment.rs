use crate::Context;

/// The segment of text we insert
pub enum Segment<E: 'static> {
	/// Just some text with 'static lifetime
    Static(&'static str),
    /// Owned string
    Owned(String),

    // The value linked to some variable of the tag
    Linked(String),

    // Your own handler
    Computed(&'static dyn Fn(&mut Context) -> Result<String, E>),
    
    /// Anything inside the tag
    Inner,
}
