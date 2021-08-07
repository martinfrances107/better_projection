use crate::InterpolateRaw;
use crate::InterpolateTrait;
use crate::Stream;
use crate::StreamNode;

#[derive(Clone, Copy, Default)]
pub struct InterpolateB {}

impl InterpolateB {
    pub fn new() -> Self {
        Self {}
    }
}
impl InterpolateRaw for InterpolateB {}
impl<SINK> InterpolateTrait for StreamNode<SINK, InterpolateB>
where
    SINK: Stream,
{
    fn interpolate(&self) -> u8 {
        dbg!("Inside Clip B");
        8
    }
}
