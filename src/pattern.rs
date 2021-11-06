use crate::Segment;

/// Insertion patterns based on segments order
pub struct Pattern<E: 'static> {
    pub segments: Vec<Segment<E>>,
}

impl<E> Pattern<E> {
    pub fn new() -> Self {
        Self {
            segments: Default::default(),
        }
    }

    pub fn segment(mut self, s: Segment<E>) -> Self {
        self.segments.push(s);
        self
    }
}
