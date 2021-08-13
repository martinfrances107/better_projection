use crate::ClipTrait;
// use crate::InterpolateRaw;
use crate::Interpolate;
use crate::Stream;
use crate::StreamNode;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Clip<ISINK>
where
    ISINK: Stream,
{
    pd: PhantomData<ISINK>,
    interpolate_fn: Interpolate<ISINK>,
}

impl<ISINK> Clip<ISINK>
where
    ISINK: Stream,
{
    pub fn new(interpolate_fn: Interpolate<ISINK>) -> Self {
        Self {
            pd: PhantomData::<ISINK>,
            interpolate_fn,
        }
    }
}
/// NB in this code ISINK and SINK are common.
/// In general they are indepedant.
impl<SINK> ClipTrait for StreamNode<SINK, Clip<SINK>>
where
    SINK: Stream,
{
    fn clip(&self) -> u8 {
        3u8
    }
}

impl<SINK> Stream for StreamNode<SINK, Clip<SINK>>
where
    SINK: Stream,
{
    fn point(&mut self, val: u8) {
        dbg!("inside point clip B");
        dbg!(val);
        let i_inc = 3u8;
        (self.raw.interpolate_fn)(10u8, 0u8, self.sink.clone());
        self.sink.borrow_mut().point(val + self.clip() + i_inc)
    }
}
