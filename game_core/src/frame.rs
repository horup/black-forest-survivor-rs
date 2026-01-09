use std::default;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Frame {
    Default,
    Walk1,
    Walk2,
}

impl default::Default for Frame {
    fn default() -> Self {
        Frame::Default
    }
}