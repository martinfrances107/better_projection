use crate::ClipTrait;
use crate::InterpolateRaw;
use crate::NodeFactory;
use crate::Stream;
use crate::StreamNode;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Clip<I, ISINK>
where
    I: InterpolateRaw,
    ISINK: Stream,
{
    pd: PhantomData<ISINK>,
    interpolate_factory: NodeFactory<ISINK, I>,
}

impl<I, ISINK> Clip<I, ISINK>
where
    I: InterpolateRaw,
    ISINK: Stream,
{
    pub fn new(interpolate_factory: NodeFactory<ISINK, I>) -> Self {
        Self {
            pd: PhantomData::<ISINK>,
            interpolate_factory,
        }
    }
}
/// NB in this code ISINK and SINK are common.
/// In general they are indepedant.
impl<I, SINK> ClipTrait for StreamNode<SINK, Clip<I, SINK>>
where
    I: InterpolateRaw,
    SINK: Stream,
{
    fn clip(&self) -> u8 {
        3u8
    }
}

impl<I, SINK> Stream for StreamNode<SINK, Clip<I, SINK>>
where
    I: InterpolateRaw + Clone,
    SINK: Stream,
{
    fn point(&mut self, val: u8) {
        dbg!("inside point clip B");
        dbg!(val);
        // let si = self.raw.interpolate_factory.generate(self.sink.clone());
        // let i_inc = si.interpolate();
        let i_inc = 3u8;

        self.sink.borrow_mut().point(val + self.clip() + i_inc)
    }
}
