use crate::Interpolate;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub fn gen_interpolate_a<STREAM>(radius: u8) -> Interpolate<STREAM>
where
    STREAM: Debug,
{
    Rc::new(move |to: u8, from: u8, stream: Rc<RefCell<STREAM>>| {
        dbg!("computed {:#?} {:#?}", radius + to + from, stream);
    })
}
