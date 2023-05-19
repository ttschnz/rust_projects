use std::collections::HashMap;

use super::Solver;
pub use super::Vertices;

#[derive(Debug)]
enum VertexState {
    Unexplored,
    Estimated(usize, Option<usize>),
    Explored(usize, Option<usize>),
}

impl VertexState {
    fn get_score(&self) -> Option<usize> {
        match self {
            VertexState::Explored(score, _) => Some(score.clone()),
            VertexState::Estimated(score, _) => Some(score.clone()),
            VertexState::Unexplored => None,
        }
    }
    fn get_previous(&self) -> Option<usize> {
        match self {
            VertexState::Explored(_, previous_label) => previous_label.clone(),
            VertexState::Estimated(_, previous_label) => previous_label.clone(),
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
    fn explore(&self, start: &usize) -> Result<HashMap<usize, VertexState>, String> {
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
                VertexState::Explored(0, None),
            );
            let mut current_vertex = current_vertex;
            // run while there are vertices that are still not completely explored
            while state
                .values()
                .find(|state| state != &&VertexState::Estimated(0, None))
                .is_some()
            {
                let current_vertex_label = current_vertex.as_ref().borrow().label();
                let current_vertex_score = match state[&current_vertex_label].get_score() {
                    Some(score) => score,
                    _ => 0,
                };

                state.insert(
                    current_vertex_label,
                    VertexState::Explored(
                        current_vertex_score,
                        state[&current_vertex_label].get_previous(),
                    ),
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
                    if state.get(&target_vertex_label) != Some(&VertexState::Explored(0, None)) {
                        state.insert(
                            target_vertex_label,
                            VertexState::Estimated(new_score, Some(current_vertex_label)),
                        );
                    }
                }

                // find the lowest unexplored value and set to current_vertex
                let mut lowest_score = None;
                let mut lowest_label = 0;
                for (next_label, next_state) in state
                    .iter()
                    .filter(|(_label, state)| state != &&VertexState::Explored(0, None))
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

                if lowest_score.is_none() {
                    return Ok(state);
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
fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return true;
        }
    }
    return false;
}
impl Solver for Vertices {
    fn shortest_path(&self, a: &usize, b: &usize) -> Vec<usize> {
        if let Ok(state) = self.explore(a) {
            // panic!("{:?}", state);

            let mut current_pointer = b.clone();
            let mut path = vec![];
            path.push(current_pointer.clone());
            while &path[path.len() - 1] != a {
                match state.get(&current_pointer).unwrap().get_previous() {
                    Some(previous) => {
                        current_pointer = previous;
                        path.push(current_pointer);
                    }
                    _ => break,
                }
                if contains_duplicate(path) {
                    return path;
                }
            }

            path.reverse();
            path
        } else {
            vec![]
        }
    }
}
