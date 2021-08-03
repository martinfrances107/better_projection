#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod projection_test {

    use std::cell::RefCell;
    use std::rc::Rc;

    // extern crate pretty_assertions;

    use better_projection::node_factory::NodeFactory;
    use better_projection::path::Path;
    use better_projection::projection_builder::ProjectionBuilder;
    use better_projection::ClipA;
    use better_projection::ClipB;
    use better_projection::Result;

    #[test]
    fn swap() {
        // assert_eq!(1, 2);
        let path = Rc::new(RefCell::new(Path::default()));
        let cfa = NodeFactory { raw: ClipA {} };
        let mut pb = ProjectionBuilder::new(cfa);
        let projection = pb.build();
        let stream_output = projection.stream(path.clone());
        stream_output.borrow_mut().point(0);

        assert_eq!(path.borrow().result(), 11);

        let cfb = NodeFactory { raw: ClipB {} };
        let pb2 = pb.clip(cfb);
        let projection2 = pb2.build();

        let path2 = Rc::new(RefCell::new(Path::default()));
        let stream_out2 = projection2.stream(path2.clone());
        stream_out2.borrow_mut().point(100);
        assert_eq!(path2.borrow_mut().result(), 211);
        assert_eq!(2 + 2, 4);
    }
}
