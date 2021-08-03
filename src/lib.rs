pub mod node_factory;
pub mod path;
mod projection;
pub mod projection_builder;
mod stream_node;

use crate::node_factory::NodeFactory;
use crate::projection::Projection;
use crate::stream_node::StreamNode;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Stream {
    fn point(&mut self, val: u8);
}

trait Clip {
    fn clip(&mut self) -> u8;
}

pub trait Result: Stream {
    fn result(self) -> u8;
}

pub trait NF {
    fn generate(&self, sink: Rc<RefCell<dyn Stream>>) -> Rc<RefCell<dyn Stream>>;
}

// NB cannot have a default value here.
#[derive(Copy, Clone)]
struct NodeRawA {
    inc_a: u8, // parent: &'a mut dyn Stream,
               // placeholder: NodeANoSink
}

impl Stream for StreamNode<NodeRawA> {
    fn point(&mut self, val: u8) {
        dbg!("inside streamNodeRawA");
        dbg!(val);
        self.sink.borrow_mut().point(val + self.raw.inc_a)
    }
}

#[derive(Copy, Clone, Default)]
pub struct ClipA {}
impl ClipA {
    fn new() -> ClipA {
        ClipA {}
    }
}

impl Clip for StreamNode<ClipA> {
    fn clip(&mut self) -> u8 {
        dbg!("Inside Clip A");
        3
    }
}

impl Stream for StreamNode<ClipA> {
    fn point(&mut self, val: u8) {
        self.clip();
        self.sink.borrow_mut().point(val);
    }
}

#[derive(Clone, Copy, Default)]
pub struct ClipB {}
impl Clip for ClipB {
    fn clip(&mut self) -> u8 {
        dbg!("Inside Clip B");
        8
    }
}
impl Stream for StreamNode<ClipB> {
    fn point(&mut self, val: u8) {
        dbg!("inside point clip B");
        dbg!(val);
        self.sink.borrow_mut().point(val + self.raw.clip())
    }
}

#[derive(Copy, Clone)]
struct NodeRawB {
    inc_b: u8,
}

impl Stream for StreamNode<NodeRawB> {
    fn point(&mut self, val: u8) {
        dbg!("inside node NodeRawB");
        dbg!(val);
        self.sink.borrow_mut().point(val + self.raw.inc_b)
    }
}
