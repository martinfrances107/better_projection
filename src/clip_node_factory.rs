use crate::clip::Clip;
use crate::Interpolate;
use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct ClipNodeFactory<SINK>
where
    SINK: Stream,
{
    pd: PhantomData<SINK>,
    interpolate_fn: Interpolate<SINK>,
}

impl<SINK> ClipNodeFactory<SINK>
where
    SINK: Stream,
{
    pub fn new(interpolate_fn: Interpolate<SINK>) -> Self {
        ClipNodeFactory {
            interpolate_fn,
            pd: PhantomData::<SINK>,
        }
    }
}

impl<SINK> NF for ClipNodeFactory<SINK>
where
    SINK: Stream,
{
    type Sink = SINK;
    type SR = Clip<SINK>;
    fn generate(&self, sink: Rc<RefCell<SINK>>) -> StreamNode<SINK, Clip<SINK>> {
        StreamNode {
            raw: Clip::new(self.interpolate_fn.clone()),
            sink,
        }
    }
}
