pub mod clip;
pub mod clip_node_factory;
pub mod node_factory;
pub mod path;
pub mod projection_builder;

pub mod interpolate_a;
pub mod interpolate_b;
mod projection;
mod stream_node;

use crate::node_factory::NodeFactory;
use crate::projection::Projection;
use crate::stream_node::StreamNode;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Stream: Clone {
    fn point(&mut self, val: u8);
}

trait ClipTrait {
    fn clip(&self) -> u8;
}

pub trait Result: Stream {
    fn result(self) -> u8;
}

pub trait NF {
    type Sink;
    type SR;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> StreamNode<Self::Sink, Self::SR>
    where
        <Self as NF>::SR: Clone,
        <Self as NF>::Sink: Stream;
}

// NB cannot have a default value here.
#[derive(Copy, Clone, Debug)]
pub struct NodeRawA {
    inc_a: u8, // parent: &'a mut dyn Stream,
               // placeholder: NodeANoSink
}

impl<SINK> Stream for StreamNode<SINK, NodeRawA>
where
    SINK: Stream,
{
    fn point(&mut self, val: u8) {
        dbg!("inside streamNodeRawA");
        dbg!(val);
        self.sink.borrow_mut().point(val + self.raw.inc_a)
    }
}

pub type Interpolate<STREAM> = Rc<dyn Fn(u8, u8, Rc<RefCell<STREAM>>)>;

#[derive(Copy, Clone, Debug)]
pub struct NodeRawB {
    inc_b: u8,
}

impl<SINK> Stream for StreamNode<SINK, NodeRawB>
where
    SINK: Stream,
{
    fn point(&mut self, val: u8) {
        dbg!("inside node NodeRawB");
        dbg!(val);
        self.sink.borrow_mut().point(val + self.raw.inc_b)
    }
}
