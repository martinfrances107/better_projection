use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

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

/// Dynamic implementation.
impl<SINK, SR> NF for NodeFactory<SINK, SR>
where
    SINK: Stream,
    SR: Clone,
{
    type Sink = SINK;
    type SR = SR;
    fn generate(&self, sink: Rc<RefCell<SINK>>) -> StreamNode<SINK, SR> {
        StreamNode {
            raw: self.raw.clone(),
            sink,
        }
    }
}
