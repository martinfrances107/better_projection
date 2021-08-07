use crate::Stream;

#[derive(Clone)]
pub struct StreamNode<SINK, SR>
where
    SINK: Stream,
    SR: Clone,
{
    pub raw: SR,
    pub sink: SINK,
}
