use crate::Projection;
use crate::NF;

pub struct ProjectionBuilder<CF>
where
    CF: NF,
{
    cf: CF,
}

impl<CF> ProjectionBuilder<CF>
where
    CF: 'static + NF + Clone,
{
    #[inline]
    pub fn new(cf: CF) -> ProjectionBuilder<CF> {
        ProjectionBuilder { cf }
    }

    #[inline]
    pub fn clip<CFNEW>(&mut self, cf: CFNEW) -> ProjectionBuilder<CFNEW>
    where
        CFNEW: NF,
    {
        ProjectionBuilder { cf }
    }

    #[inline]
    pub fn build(&self) -> Projection<CF> {
        Projection::new(self.cf.clone())
    }
}
