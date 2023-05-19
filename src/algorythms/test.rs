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

        let e = Edge::new(&2, &a, &b);
        assert_eq!(e.vertices().0.as_ref().borrow().label(), 0);
        assert_eq!(e.vertices().1.as_ref().borrow().label(), 1);
    }
    #[test]
    fn can_create_bidirectional_connections() {
        let mut vertices = Vertices::new();

        vertices
            .create_and_insert(&0)
            .expect("Could not create vertex");
        vertices
            .create_and_insert(&1)
            .expect("Could not create vertex");

        vertices.add_connection(&0, &1, &5, Direction::Bidirectional);

        for label in 0..=1 {
            match vertices.find(&label) {
                Some(vertex) => {
                    assert_eq!(
                        vertex.as_ref().borrow().edges().len(),
                        1,
                        "vertex {} did not have 1 edge",
                        label
                    );
                    assert_eq!(
                        vertex.as_ref().borrow().adjacent_vertices()[0]
                            .as_ref()
                            .borrow()
                            .label(),
                        label ^ 1,
                        "adjacent vertex to {} was not {}",
                        label,
                        label ^ 1
                    );
                }
                _ => panic!("vertex {} not found", label),
            }
        }
    }

    #[test]
    fn can_create_forward_connections() {
        let mut vertices = Vertices::new();

        vertices
            .create_and_insert(&0)
            .expect("Could not create vertex");

        vertices
            .create_and_insert(&1)
            .expect("Could not create vertex");

        vertices.add_connection(&0, &1, &5, Direction::Forward);

        // vertex 0 should only have one edge
        if let Some(vertex) = vertices.get(&0) {
            let vertex = vertex.as_ref().borrow();
            let (edges, label) = (vertex.edges().len(), vertex.label());
            assert_eq!(
                edges, 1,
                "vertex {} does not have 1 edge, instead it has {}",
                label, edges
            );
            // the adjacent vertex to 0 should be 1
            assert_eq!(vertex.adjacent_vertices()[0].as_ref().borrow().label(), 1)
        }

        // vertex 1 should have exactly 0 edges
        if let Some(vertex) = vertices.get(&1) {
            let vertex = vertex.as_ref().borrow();
            let (edges, label) = (vertex.edges().len(), vertex.label());
            assert_eq!(
                edges,
                0,
                "vertex {} does not have 0 edges, instead it has {}: {:?}",
                label,
                edges,
                vertex.edges()
            )
        }
    }

    #[test]
    fn can_create_reverse_connections() {
        let mut vertices = Vertices::new();

        vertices
            .create_and_insert(&0)
            .expect("Could not create vertex");

        vertices
            .create_and_insert(&1)
            .expect("Could not create vertex");

        vertices.add_connection(&0, &1, &5, Direction::Reverse);

        // vertex 1 should only have one edge
        if let Some(vertex) = vertices.get(&1) {
            let vertex = vertex.as_ref().borrow();
            let (edges, label) = (vertex.edges().len(), vertex.label());
            assert_eq!(
                edges, 1,
                "vertex {} does not have 1 edge, instead it has {}",
                label, edges
            );
            // the adjacent vertex to 1 should be 0
            assert_eq!(vertex.adjacent_vertices()[0].as_ref().borrow().label(), 0)
        }

        // vertex 0 should have exactly 0 edges
        if let Some(vertex) = vertices.get(&0) {
            let vertex = vertex.as_ref().borrow();
            let (edges, label) = (vertex.edges().len(), vertex.label());
            assert_eq!(
                edges,
                0,
                "vertex {} does not have 0 edges, instead it has {}: {:?}",
                label,
                edges,
                vertex.edges()
            )
        }
    }
    #[test]
    fn can_translate_config() {
        let config = vec![vec![1, 2, 3], vec![3], vec![1, 3], vec![0, 1, 2]];
        let vertices = Vertices::from_config(config.clone());

        config.iter().enumerate().for_each(|(label, connections)| {
            let from = match vertices.find(&label) {
                Some(vertex) => vertex,
                _ => panic!("could not find vertex with label {}", label),
            };

            let adjacent_vertices = from.as_ref().borrow().adjacent_vertices();
            // check that all that exist are in config
            let unasked_connections = adjacent_vertices
                .iter()
                .filter(|vertex| !connections.contains(&vertex.as_ref().borrow().label()))
                .collect::<Vec<&Rc<RefCell<Vertex>>>>();
            assert_eq!(
                0,
                unasked_connections.len(),
                "some connections exist that were not asked for: {:?}. connections={:?}, vertex={:?}",
                unasked_connections
                    .iter()
                    .map(|vertex| { vertex.as_ref().borrow().label() })
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
