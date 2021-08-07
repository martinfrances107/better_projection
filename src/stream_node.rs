use crate::Stream;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct StreamNode<SINK, SR>
where
    SINK: Stream,
    SR: Clone,
{
    pub raw: SR,
    pub sink: Rc<RefCell<SINK>>,
}
