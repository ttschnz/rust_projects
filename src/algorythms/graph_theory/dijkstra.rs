use std::collections::HashMap;

use super::Solver;
pub use super::Vertices;

#[derive(Debug)]
pub enum VertexState {
    Unexplored,
    Estimated {
        current_score: usize,
        coming_from: Option<usize>,
    },
    Explored {
        score: usize,
        coming_from: Option<usize>,
    },
}

impl VertexState {
    fn is_explored(&self) -> bool {
        match self {
            VertexState::Explored { .. } => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_estimated(&self) -> bool {
        match self {
            VertexState::Estimated { .. } => true,
            _ => false,
        }
    }
    #[allow(dead_code)]
    fn is_unexplored(&self) -> bool {
        match self {
            VertexState::Unexplored => true,
            _ => false,
        }
    }
    fn get_score(&self) -> Option<usize> {
        match self {
            VertexState::Explored { score, .. } => Some(score.clone()),
            VertexState::Estimated { current_score, .. } => Some(current_score.clone()),
            VertexState::Unexplored => None,
        }
    }
    fn get_path_backwards(&self) -> Option<usize> {
        match self {
            VertexState::Explored { coming_from, .. } => coming_from.clone(),
            VertexState::Estimated { coming_from, .. } => coming_from.clone(),
            VertexState::Unexplored => None,
        }
    }
}

impl PartialEq for VertexState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Vertices {
    fn explore(&self, start: &usize, end: &usize) -> Result<HashMap<usize, VertexState>, String> {
        let mut state = HashMap::new();

        // make all unexplored
        self.vertices.keys().for_each(|key| {
            state.insert(key.clone(), VertexState::Unexplored);
        });

        let current_vertex = self.find(start);
        if let Some(current_vertex) = current_vertex {
            // start at score 0
            state.insert(
                current_vertex.as_ref().borrow().label(),
                VertexState::Explored {
                    score: 0,
                    coming_from: None,
                },
            );
            let mut current_vertex = current_vertex;

            // run while there are vertices that are still not completely explored
            while state
                .get(end)
                .and_then(|targets_state| {
                    if targets_state.is_explored() {
                        None //=> done, do nothing more
                    } else {
                        Some(true) //=> keep exploring
                    }
                })
                .is_some()
            {
                let current_vertex_label = current_vertex.as_ref().borrow().label();
                let current_vertex_score = match state[&current_vertex_label].get_score() {
                    Some(score) => score,
                    _ => 0,
                };

                state.insert(
                    current_vertex_label,
                    VertexState::Explored {
                        score: current_vertex_score,
                        coming_from: state[&current_vertex_label].get_path_backwards(),
                    },
                );

                // update all surrounding values according to current_vertex
                for edge in current_vertex.as_ref().borrow().edges() {
                    let target_vertex_label = edge.as_ref().borrow().to.as_ref().borrow().label();
                    let new_potential_score =
                        current_vertex_score + edge.as_ref().borrow().weight();
                    let new_score =
                        if let Some(comparing_score) = state[&target_vertex_label].get_score() {
                            std::cmp::max(comparing_score, new_potential_score)
                        } else {
                            new_potential_score
                        };
                    if state
                        .get(&target_vertex_label)
                        .and_then(|target_state| {
                            if target_state.is_explored() {
                                None
                            } else {
                                Some(true)
                            }
                        })
                        .is_some()
                    {
                        state.insert(
                            target_vertex_label,
                            VertexState::Estimated {
                                coming_from: Some(current_vertex_label),
                                current_score: new_score,
                            },
                        );
                    }
                }

                // find the lowest unexplored value and set to current_vertex

                let mut lowest_score = None;
                let mut lowest_label = 0;
                for (next_label, next_state) in
                    state.iter().filter(|(_label, state)| !state.is_explored())
                {
                    if let Some(value) = next_state.get_score() {
                        if match lowest_score {
                            Some(lowest_score_value) => lowest_score_value > value,
                            _ => true,
                        } {
                            lowest_score = Some(value);
                            lowest_label = next_label.clone();
                        }
                    }
                }

                if lowest_score.is_none()
                    && state
                        .values()
                        .filter(|state| state.is_unexplored())
                        .collect::<Vec<&VertexState>>()
                        .len()
                        > 0
                {
                    return Err(format!(
                        "there is no way for getting from {} to {}. There is a loop.",
                        start, end
                    ));
                }

                if let Some(lowest_vertex) = self.find(&lowest_label) {
                    current_vertex = lowest_vertex;
                } else {
                    return Err(format!(
                        "could not find vertex for lowest label: {}",
                        lowest_label
                    ));
                }
            }

            Ok(state)
        } else {
            Err("could not find start vertex".to_string())
        }
    }
}

impl Solver for Vertices {
    fn shortest_path(&self, a: &usize, b: &usize) -> Result<Vec<usize>, String> {
        match self.explore(a, b) {
            Ok(state) => {
                let mut current_pointer = b.clone();
                let mut path = vec![];
                path.push(current_pointer.clone());
                while &path[path.len() - 1] != a {
                    match state.get(&current_pointer).unwrap().get_path_backwards() {
                        Some(previous) => {
                            current_pointer = previous;
                            path.push(current_pointer);
                        }
                        None => {
                            println!(
                                "could not reverse path from {:?}. state: {:?}",
                                current_pointer, state
                            );
                            break;
                        }
                    }
                }

                path.reverse();
                Ok(path)
            }
            Err(content) => Err(content),
        }
    }
}
