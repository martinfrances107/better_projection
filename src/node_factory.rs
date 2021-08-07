use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct NodeFactory<SINK, SR>
where
    SINK: Stream,
{
    pd: PhantomData<SINK>,
    pub raw: SR,
}

impl<SINK, SR> NodeFactory<SINK, SR>
where
    SINK: Stream,
{
    pub fn new(raw: SR) -> NodeFactory<SINK, SR> {
        NodeFactory {
            pd: PhantomData::<SINK>,
            raw,
        }
    }
}
impl<SINK, SR> NF for NodeFactory<SINK, SR>
where
    SINK: Stream,
    SR: Clone,
{
    type Sink = SINK;
    type SR = SR;
    fn generate(&self, sink: SINK) -> StreamNode<SINK, SR> {
        StreamNode {
            raw: self.raw.clone(),
            sink,
        }
    }
}
