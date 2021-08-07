use crate::InterpolateRaw;
use crate::InterpolateTrait;
use crate::Stream;
use crate::StreamNode;

#[derive(Copy, Clone, Default)]
pub struct InterpolateA {
    val: u8,
}
impl InterpolateA {
    pub fn new(val: u8) -> InterpolateA {
        InterpolateA { val }
    }
}

impl InterpolateRaw for InterpolateA {}

impl<SINK> InterpolateTrait for StreamNode<SINK, InterpolateA>
where
    SINK: Stream,
{
    fn interpolate(&self) -> u8 {
        dbg!("inside interpolate() - InterpolateA");
        self.raw.val
    }
}
