#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod projection_test {

    use better_projection::clip_node_factory::ClipNodeFactory;
    use better_projection::interpolate_a::InterpolateA;
    use better_projection::interpolate_b::InterpolateB;
    use better_projection::node_factory::NodeFactory;
    use better_projection::path::Path;
    use better_projection::projection_builder::ProjectionBuilder;
    use better_projection::Result;
    use better_projection::Stream;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn swap() {
        let path = Rc::new(RefCell::new(Path::default()));
        let ia = InterpolateA::new(3u8);
        let interpolate_factory_a = NodeFactory::new(ia);
        let cfa = ClipNodeFactory::new(interpolate_factory_a);
        let mut pb = ProjectionBuilder::new(cfa);
        let projection = pb.build();
        let mut stream_output = projection.stream(path.clone());
        stream_output.point(0);

        assert_eq!(path.borrow().result(), 11);

        let ib = InterpolateB::new();
        let interpolate_factory_b = NodeFactory::new(ib);
        let cfb = ClipNodeFactory::new(interpolate_factory_b);
        let pb2 = pb.swap_clip(cfb);
        let projection2 = pb2.build();

        let path2 = Rc::new(RefCell::new(Path::default()));
        let mut stream_out2 = projection2.stream(path2.clone());
        stream_out2.point(100);
        assert_eq!(path2.borrow().result(), 211);
        assert_eq!(2 + 2, 4);
    }
}
