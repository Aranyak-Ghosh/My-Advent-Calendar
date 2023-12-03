use std::fmt::Debug;

pub struct Answer {
    inner: Box<dyn std::fmt::Debug>,
}

impl From<u128> for Answer {
    fn from(val: u128) -> Self {
        Answer {
            inner: Box::new(val),
        }
    }
}

impl Debug for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Answer")
            .field("value", &self.inner)
            .finish()
    }
}

pub trait Solution {
    fn part_a(&self) -> Answer;

    fn part_b(&self) -> Answer;
}
