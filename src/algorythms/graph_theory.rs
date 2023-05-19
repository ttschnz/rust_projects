use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    ops::Deref,
    rc::Rc,
    slice::Iter,
};

pub mod dijkstra;

#[derive(PartialEq)]
pub struct Edge {
    weight: usize,
    from: Rc<RefCell<Vertex>>,
    to: Rc<RefCell<Vertex>>,
}

impl Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge{{ weight: {}, from: {}, to: {} }}",
            self.weight(),
            self.from.as_ref().borrow().label(),
            self.to.as_ref().borrow().label(),
        )
    }
}

impl Edge {
    pub fn new(weight: usize, from: &Rc<RefCell<Vertex>>, to: &Rc<RefCell<Vertex>>) -> Edge {
        Edge {
            weight,
            from: Rc::clone(from),
            to: Rc::clone(to),
        }
    }

    pub fn weight(&self) -> &usize {
        &self.weight
    }

    pub fn vertices(&self) -> (Rc<RefCell<Vertex>>, Rc<RefCell<Vertex>>) {
        (Rc::clone(&self.from), Rc::clone(&self.to))
    }
}
/// #Vertex
/// Label must be unique!
#[derive(Debug)]
pub struct Vertex {
    label: usize,
    edges: Vec<Rc<RefCell<Edge>>>,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
    }

    fn ne(&self, other: &Self) -> bool {
        self.label() != other.label()
    }
}

impl Vertex {
    pub fn new(label: &usize) -> Vertex {
        Vertex {
            label: label.clone(),
            edges: vec![],
        }
    }

    pub fn add_edge(&mut self, edge: &Rc<RefCell<Edge>>) {
        self.edges.push(Rc::clone(edge));
    }

    pub fn label(&self) -> usize {
        self.label
    }

    pub fn edges(&self) -> Iter<Rc<RefCell<Edge>>> {
        self.edges.iter()
    }

    pub fn adjacent_vertices(&self) -> Vec<Rc<RefCell<Vertex>>> {
        self.edges()
            .filter(|edge| edge.borrow().from.as_ref().borrow().deref() == self)
            .map(|edge| Rc::clone(&edge.borrow().to))
            .collect::<Vec<Rc<RefCell<Vertex>>>>()
    }
}

///
/// Creates many Vertices with Edges in between
///
pub fn vertices_from_config(config: Vec<Vec<usize>>) -> Vec<Rc<RefCell<Vertex>>> {
    let mut vertex_map = HashMap::new();
    config
        .iter()
        .enumerate()
        .for_each(|(current_index, sub_config)| {
            let from = Rc::clone(
                vertex_map
                    .entry(current_index.clone())
                    .or_insert(Rc::new(RefCell::new(Vertex::new(&current_index.clone())))),
            );
            sub_config.iter().for_each(|target_index| {
                let to = Rc::clone(
                    vertex_map
                        .entry(target_index.clone())
                        .or_insert(Rc::new(RefCell::new(Vertex::new(&target_index.clone())))),
                );
                add_connection(&from, &to, 1, Direction::Forward);
            })
        });
    vertex_map
        .values()
        .map(|vertex| Rc::clone(vertex))
        .collect()
}

pub fn add_connection(
    from: &Rc<RefCell<Vertex>>,
    to: &Rc<RefCell<Vertex>>,
    weight: usize,
    direction: Direction,
) -> Vec<Rc<RefCell<Edge>>> {
    let start = Rc::clone(from);
    let end = Rc::clone(to);
    let edges = match direction {
        Direction::Forward => {
            vec![Rc::new(RefCell::new(Edge::new(weight, &start, &end)))]
        }
        Direction::Reverse => {
            vec![Rc::new(RefCell::new(Edge::new(weight, &end, &start)))]
        }
        Direction::Bidirectional => {
            vec![
                Rc::new(RefCell::new(Edge::new(weight, &start, &end))),
                Rc::new(RefCell::new(Edge::new(weight, &end, &start))),
            ]
        }
    };
    edges.iter().for_each(|edge| {
        start.borrow_mut().add_edge(&edge);
        end.borrow_mut().add_edge(&edge);
    });

    edges
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
    #[default]
    Bidirectional,
}
