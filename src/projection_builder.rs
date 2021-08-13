use crate::clip_node_factory::ClipNodeFactory;
use crate::NodeRawA;
use crate::NodeRawB;
use crate::Projection;
use crate::Stream;
use crate::StreamNode;
use std::marker::PhantomData;

pub struct ProjectionBuilder<DRAIN>
where
    DRAIN: Stream,
{
    clip_node_factory: ClipNodeFactory<StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    pd: PhantomData<DRAIN>,
}

impl<DRAIN> ProjectionBuilder<DRAIN>
where
    DRAIN: Stream,
{
    #[inline]
    pub fn new(
        clip_node_factory: ClipNodeFactory<StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    ) -> ProjectionBuilder<DRAIN> {
        ProjectionBuilder {
            clip_node_factory,
            pd: PhantomData::<DRAIN>,
        }
    }

    #[inline]
    pub fn swap_clip(
        &mut self,
        clip_node_factory: ClipNodeFactory<StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    ) -> ProjectionBuilder<DRAIN> {
        ProjectionBuilder {
            clip_node_factory,
            pd: PhantomData::<DRAIN>,
        }
    }

    #[inline]
    pub fn build(&self) -> Projection<DRAIN> {
        Projection::new(self.clip_node_factory.clone())
    }
}
