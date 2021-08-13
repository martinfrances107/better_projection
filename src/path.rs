use crate::Result;
use crate::Stream;

#[derive(Copy, Clone, Debug, Default)]
pub struct Path {
    val: u8,
}

impl Stream for Path {
    fn point(&mut self, val: u8) {
        dbg!("inside path");
        dbg!(val);
        self.val = val;
    }
}

impl Result for Path {
    fn result(self) -> u8 {
        self.val
    }
}
