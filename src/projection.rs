use std::cell::RefCell;
use std::rc::Rc;

use crate::NodeFactory;
use crate::NodeRawA;
use crate::NodeRawB;
use crate::Stream;
use crate::NF;

pub struct Projection<CF>
where
    CF: NF,
{
    af: NodeFactory<NodeRawA>,
    bf: NodeFactory<NodeRawB>,
    cf: CF,
    cache: Option<Rc<RefCell<dyn Stream>>>,
}

impl<CF> Clone for Projection<CF>
where
    CF: NF + Copy,
{
    fn clone(&self) -> Self {
        match &self.cache {
            Some(cache) => Self {
                af: self.af,
                bf: self.bf,
                cf: self.cf,
                cache: Some(cache.clone()),
            },

            None => Self {
                af: self.af,
                bf: self.bf,
                cf: self.cf,
                cache: None,
            },
        }
    }
}

impl<CF> Projection<CF>
where
    CF: 'static + NF,
{
    pub fn new(c: CF) -> Projection<CF> {
        dbg!("in  projection::new()");
        Projection {
            af: NodeFactory::new(NodeRawA { inc_a: 1 }),
            bf: NodeFactory::new(NodeRawB { inc_b: 10 }),
            cf: c,
            cache: None,
        }
    }

    pub fn stream(mut self, drain: Rc<RefCell<dyn Stream>>) -> Rc<RefCell<dyn Stream>> {
        match self.cache {
            None => {
                let a = self.af.generate(drain);
                let b = self.bf.generate(a);
                let c = self.cf.generate(b);
                self.cache = Some(c.clone());
                c
            }
            Some(c) => c.clone(),
        }
    }
}
