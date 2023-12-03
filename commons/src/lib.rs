use std::fmt::Debug;

#[derive(PartialEq)]
pub struct Answer<T> {
    inner: T,
}

impl<T> Answer<T> {
    pub fn new(val: T) -> Self {
        Self { inner: val }
    }
}

impl<T: Debug + PartialEq> Debug for Answer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Answer")
            .field("value", &self.inner)
            .finish()
    }
}

pub trait Solution {
    type Item: Debug + PartialEq;

    fn part_a(&self) -> Answer<Self::Item>;

    fn part_b(&self) -> Answer<Self::Item>;
}
