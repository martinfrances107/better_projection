use crate::Interpolate;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub fn gen_interpolate_b<STREAM>() -> Interpolate<STREAM>
where
    STREAM: Debug,
{
    let internal_radius = 1u8;
    Rc::new(move |to: u8, from: u8, stream: Rc<RefCell<STREAM>>| {
        dbg!("computed {:#?} {:#?}", internal_radius + to + from, stream);
    })
}
