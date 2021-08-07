use crate::clip_node_factory::ClipNodeFactory;
use crate::InterpolateRaw;
use crate::NodeRawA;
use crate::NodeRawB;
use crate::Projection;
use crate::Stream;
use crate::StreamNode;
use std::marker::PhantomData;

pub struct ProjectionBuilder<I, DRAIN>
where
    I: InterpolateRaw,
    DRAIN: Stream,
{
    clip_node_factory: ClipNodeFactory<I, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    pd: PhantomData<DRAIN>,
    pi: PhantomData<I>,
}

impl<I, DRAIN> ProjectionBuilder<I, DRAIN>
where
    I: InterpolateRaw,
    DRAIN: Stream,
{
    #[inline]
    pub fn new(
        clip_node_factory: ClipNodeFactory<I, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    ) -> ProjectionBuilder<I, DRAIN> {
        ProjectionBuilder {
            clip_node_factory,
            pd: PhantomData::<DRAIN>,
            pi: PhantomData::<I>,
        }
    }

    #[inline]
    pub fn swap_clip<INEW>(
        &mut self,
        clip_node_factory: ClipNodeFactory<INEW, StreamNode<StreamNode<DRAIN, NodeRawA>, NodeRawB>>,
    ) -> ProjectionBuilder<INEW, DRAIN>
    where
        INEW: InterpolateRaw,
    {
        ProjectionBuilder {
            clip_node_factory,
            pd: PhantomData::<DRAIN>,
            pi: PhantomData::<INEW>,
        }
    }

    #[inline]
    pub fn build(&self) -> Projection<I, DRAIN> {
        Projection::new(self.clip_node_factory.clone())
    }
}
