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
    pub fn new(weight: &usize, from: &Rc<RefCell<Vertex>>, to: &Rc<RefCell<Vertex>>) -> Edge {
        Edge {
            weight: weight.clone(),
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

#[derive(Debug)]
pub struct Vertices {
    vertices: HashMap<usize, Rc<RefCell<Vertex>>>,
    // edges:
}
#[derive(Debug)]
pub enum VerticesManipulationError {
    Duplicate,
    NotFoundAfterInsert,
    NotFound,
}

impl Deref for Vertices {
    type Target = HashMap<usize, Rc<RefCell<Vertex>>>;
    fn deref(&self) -> &Self::Target {
        &self.vertices
    }
}

impl Vertices {
    pub fn new() -> Vertices {
        Vertices {
            vertices: HashMap::new(),
        }
    }
    pub fn find(&self, label: &usize) -> Option<Rc<RefCell<Vertex>>> {
        match self.vertices.get(label) {
            Some(vertex) => Some(Rc::clone(vertex)),
            _ => None,
        }
    }
    pub fn insert(
        &mut self,
        vertex: Vertex,
    ) -> Result<Rc<RefCell<Vertex>>, VerticesManipulationError> {
        let label = vertex.label();
        let vertex = Rc::new(RefCell::new(vertex));
        if self.find(&vertex.borrow().label()).is_some() {
            return Err(VerticesManipulationError::Duplicate);
        }
        self.vertices.insert(label, vertex);
        match self.find(&label) {
            Some(r) => Ok(r),
            None => Err(VerticesManipulationError::NotFoundAfterInsert),
        }
    }

    pub fn create_and_insert(
        &mut self,
        label: &usize,
    ) -> Result<Rc<RefCell<Vertex>>, VerticesManipulationError> {
        let vertex = Vertex::new(label);
        self.insert(vertex)
    }

    pub fn find_or_create(&mut self, label: &usize) -> Rc<RefCell<Vertex>> {
        Rc::clone(
            self.vertices
                .entry(label.clone())
                .or_insert(Rc::new(RefCell::new(Vertex::new(&label)))),
        )
    }

    pub fn from_config(config: Vec<Vec<usize>>) -> Result<Vertices, VerticesManipulationError> {
        let mut vertices = Vertices::new();
        let mut error = None;
        config
            .iter()
            .enumerate()
            .for_each(|(current_index, sub_config)| {
                vertices.find_or_create(&current_index);
                sub_config.iter().for_each(|target_index| {
                    vertices.find_or_create(target_index);
                    if let Err(err) = vertices.add_connection(
                        &current_index,
                        &target_index,
                        &1,
                        Direction::Forward,
                    ) {
                        error = Some(err);
                    }
                })
            });
        if let Some(err) = error {
            Err(err)
        } else {
            Ok(vertices)
        }
    }

    pub fn add_connection(
        &self,
        from: &usize,
        to: &usize,
        weight: &usize,
        direction: Direction,
    ) -> Result<(), VerticesManipulationError> {
        match direction {
            Direction::Bidirectional => self
                .add_connection(from, to, weight, Direction::Forward)
                .and(self.add_connection(from, to, weight, Direction::Reverse)),
            Direction::Reverse => self.add_connection(to, from, weight, Direction::Forward),
            Direction::Forward => {
                if let (Some(vertex_from), Some(vertex_to)) = (self.find(from), self.find(to)) {
                    let edge = Edge::new(weight, &vertex_from, &vertex_to);
                    edge.vertices()
                        .0
                        .borrow_mut()
                        .add_edge(&Rc::new(RefCell::new(edge)));
                    Ok(())
                } else {
                    Err(VerticesManipulationError::NotFound)
                }
            }
        }
    }
}
pub trait Solver {
    /// finds the shortest path from a to b (by their IDs) and returns the path (also using IDs of vertices).
    fn shortest_path(&self, a: &usize, b: &usize) -> Result<Vec<usize>, String>;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
    #[default]
    Bidirectional,
}
