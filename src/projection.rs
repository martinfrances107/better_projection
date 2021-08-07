use crate::clip::Clip;
use crate::clip_node_factory::ClipNodeFactory;
use crate::InterpolateRaw;
use crate::NodeFactory;
use crate::NodeRawA;
use crate::NodeRawB;
use crate::Stream;
use crate::StreamNode;
use crate::NF;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Projection<I, DRAIN>
where
    I: InterpolateRaw,
    DRAIN: Stream,
{
    pd: PhantomData<DRAIN>,
    af: NodeFactory<DRAIN, NodeRawA>,
    bf: NodeFactory<StreamNode<DRAIN, NodeRawA>, NodeRawB>,
    cf: ClipNodeFactory<I, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    // cache: Option<StreamNode<StreamNode<DRAIN, NodeRawA>, Clip<I, DRAIN>>>,
}

impl<I, DRAIN> Clone for Projection<I, DRAIN>
where
    I: InterpolateRaw,
    DRAIN: Stream,
{
    fn clone(&self) -> Self {
        Self {
            pd: PhantomData::<DRAIN>,
            af: self.af.clone(),
            bf: self.bf.clone(),
            cf: self.cf.clone(),
        }
    }
}

impl<I, DRAIN> Projection<I, DRAIN>
where
    I: InterpolateRaw,
    DRAIN: Stream,
{
    pub fn new(
        cf: ClipNodeFactory<I, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    ) -> Projection<I, DRAIN> {
        dbg!("in  projection::new()");
        Projection {
            pd: PhantomData::<DRAIN>,
            af: NodeFactory::new(NodeRawA { inc_a: 1 }),
            bf: NodeFactory::new(NodeRawB { inc_b: 10 }),
            cf,
            // cache: None,
        }
    }

    pub fn stream(
        self,
        drain: Rc<RefCell<DRAIN>>,
    ) -> StreamNode<
        StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>,
        Clip<I, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    > {
        let a = Rc::new(RefCell::new(self.af.generate(drain)));
        let b = Rc::new(RefCell::new(self.bf.generate(a)));
        self.cf.generate(b)
    }
}
