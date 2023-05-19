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
pub enum VerticesInsertionError {
    Duplicate,
    NotFoundAfterInsert,
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
    ) -> Result<Rc<RefCell<Vertex>>, VerticesInsertionError> {
        let label = vertex.label();
        let vertex = Rc::new(RefCell::new(vertex));
        if self.find(&vertex.borrow().label()).is_some() {
            return Err(VerticesInsertionError::Duplicate);
        }
        self.vertices.insert(label, vertex);
        match self.find(&label) {
            Some(r) => Ok(r),
            None => Err(VerticesInsertionError::NotFoundAfterInsert),
        }
    }

    pub fn create_and_insert(
        &mut self,
        label: &usize,
    ) -> Result<Rc<RefCell<Vertex>>, VerticesInsertionError> {
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

    pub fn from_config(config: Vec<Vec<usize>>) -> Vertices {
        let mut vertices = Vertices::new();
        config
            .iter()
            .enumerate()
            .for_each(|(current_index, sub_config)| {
                vertices.find_or_create(&current_index);
                sub_config.iter().for_each(|target_index| {
                    vertices.find_or_create(target_index);
                    vertices.add_connection(&current_index, &target_index, &1, Direction::Forward);
                })
            });
        vertices
    }
    pub fn add_connection(&self, from: &usize, to: &usize, weight: &usize, direction: Direction) {
        match (self.find(from), self.find(to)) {
            (Some(start), Some(end)) => {
                if let Some(edge) = match direction {
                    Direction::Forward => {
                        Some(Rc::new(RefCell::new(Edge::new(weight, &start, &end))))
                    }
                    Direction::Reverse => {
                        // reverse goes from end to start
                        Some(Rc::new(RefCell::new(Edge::new(weight, &end, &start))))
                    }
                    Direction::Bidirectional => {
                        // bidirectional has two edges, one forward and another reverse
                        self.add_connection(from, to, weight, Direction::Forward);
                        self.add_connection(from, to, weight, Direction::Reverse);
                        None
                    }
                } {
                    edge.borrow().vertices().0.borrow_mut().add_edge(&edge)
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
    #[default]
    Bidirectional,
}
