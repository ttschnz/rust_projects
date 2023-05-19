#[cfg(test)]
mod graph_theory {
    use super::super::graph_theory::*;
    use std::{cell::RefCell, rc::Rc};
    #[test]
    fn can_create_vertex() {
        let a = Vertex::new(&0);
        assert_eq!(a.label(), 0);

        let b = Vertex::new(&1);
        assert_eq!(b.label(), 1);
    }

    #[test]
    fn can_create_edges() {
        let a = Rc::new(RefCell::new(Vertex::new(&0)));
        let b = Rc::new(RefCell::new(Vertex::new(&1)));

        let e = Edge::new(2, &a, &b);
        assert_eq!(e.vertices().0.as_ref().borrow().label(), 0);
        assert_eq!(e.vertices().1.as_ref().borrow().label(), 1);
    }
    #[test]
    fn can_create_bidirectional_connections() {
        let a = Rc::new(RefCell::new(Vertex::new(&0)));
        let b = Rc::new(RefCell::new(Vertex::new(&1)));

        let _e = add_connection(&a, &b, 5, Direction::Bidirectional);

        assert_eq!(a.borrow().edges().len(), 2);
        assert_eq!(b.borrow().edges().len(), 2);

        assert_eq!(a.borrow().adjacent_vertices()[0].borrow().label(), 1);
        assert_eq!(b.borrow().adjacent_vertices()[0].borrow().label(), 0);
    }

    #[test]
    fn can_create_forward_connections() {
        let a = Rc::new(RefCell::new(Vertex::new(&0)));
        let b = Rc::new(RefCell::new(Vertex::new(&1)));

        let _e = add_connection(&a, &b, 5, Direction::Forward);

        assert_eq!(a.borrow().edges().len(), 1);
        assert_eq!(b.borrow().edges().len(), 1);

        assert_eq!(a.borrow().adjacent_vertices()[0].borrow().label(), 1);
        assert_eq!(b.borrow().adjacent_vertices().len(), 0);
    }

    #[test]
    fn can_create_reverse_connections() {
        let a = Rc::new(RefCell::new(Vertex::new(&0)));
        let b = Rc::new(RefCell::new(Vertex::new(&1)));

        let _e = add_connection(&a, &b, 5, Direction::Reverse);

        assert_eq!(a.borrow().edges().len(), 1);
        assert_eq!(b.borrow().edges().len(), 1);

        assert_eq!(a.borrow().adjacent_vertices().len(), 0);
        assert_eq!(b.borrow().adjacent_vertices()[0].borrow().label(), 0);
    }
    #[test]
    fn can_translate_config() {
        let config = vec![vec![1, 2, 3], vec![3], vec![1, 3], vec![0, 1, 2]];
        let vertices = vertices_from_config(config.clone());

        config.iter().enumerate().for_each(|(label, connections)| {
            let from = vertices
                .iter()
                .find(|vertex| vertex.borrow().label() == label);

            let from = match from {
                Some(vertex) => vertex.borrow(),
                _ => panic!("could not find vertex with label {}", label),
            };

            let adjacent_vertices = from.adjacent_vertices();
            // check that all that exist are in config
            let unasked_connections = adjacent_vertices
                .iter()
                .filter(|vertex| !connections.contains(&vertex.borrow().label()))
                .collect::<Vec<&Rc<RefCell<Vertex>>>>();
            assert_eq!(
                0,
                unasked_connections.len(),
                "some connections exist that were not asked for: {:?}. connections={:?}, vertex={:?}",
                unasked_connections
                    .iter()
                    .map(|vertex| { vertex.borrow().label() })
                    .collect::<Vec<usize>>(),

                connections,
                from
            );

            // check that all from config exist
            let unresolved_vertices = connections
                .iter()
                .filter(|target_label| {
                    !adjacent_vertices.contains(&Rc::new(RefCell::new(Vertex::new(target_label))))
                })
                .collect::<Vec<&usize>>();
            assert_eq!(
                0,
                unresolved_vertices.len(),
                "some connections that were asked for don't exist: {:?}. connections={:?}, vertex={:?}",
                unresolved_vertices,
                connections,
                from
            );
        });
    }
}
