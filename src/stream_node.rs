use crate::Stream;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StreamNode<SR> {
    pub raw: SR,
    pub sink: Rc<RefCell<dyn Stream>>,
}
