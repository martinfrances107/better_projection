use crate::clip::Clip;
use crate::node_factory::NodeFactory;
use crate::InterpolateRaw;
use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct ClipNodeFactory<I, SINK>
where
    I: InterpolateRaw,
    SINK: Stream,
{
    pd: PhantomData<SINK>,
    interpolate_factory: NodeFactory<SINK, I>,
}

impl<I, SINK> ClipNodeFactory<I, SINK>
where
    I: InterpolateRaw,
    SINK: Stream,
{
    pub fn new(interpolate_factory: NodeFactory<SINK, I>) -> Self {
        ClipNodeFactory {
            interpolate_factory,
            pd: PhantomData::<SINK>,
            // raw: Clip{interpolate:
        }
    }
}

impl<I, SINK> NF for ClipNodeFactory<I, SINK>
where
    I: InterpolateRaw,
    SINK: Stream,
{
    type Sink = SINK;
    type SR = Clip<I, SINK>;
    fn generate(&self, sink: Rc<RefCell<SINK>>) -> StreamNode<SINK, Clip<I, SINK>> {
        StreamNode {
            raw: Clip::new(self.interpolate_factory.clone()),
            sink,
        }
    }
}
