use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct NodeFactory<SR>
where
    StreamNode<SR>: Stream,
{
    pub raw: SR,
}

impl<SR> NodeFactory<SR>
where
    SR: 'static,
    StreamNode<SR>: Stream,
{
    pub fn new(raw: SR) -> NodeFactory<SR> {
        NodeFactory { raw }
    }
}
impl<SR> NF for NodeFactory<SR>
where
    SR: 'static + Copy,
    StreamNode<SR>: Stream,
{
    fn generate(&self, sink: Rc<RefCell<dyn Stream>>) -> Rc<RefCell<dyn Stream>> {
        Rc::new(RefCell::new(StreamNode {
            raw: self.raw,
            sink,
        }))
    }
}
