use std::collections::HashMap;

use itertools::Itertools;

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
        for key in self.vertices.keys() {
            state.insert(key.clone(), VertexState::Unexplored);
        }

        // start at the vertex with id `start`
        // let current_vertex = if let Some(starting_vertex) = self.find(start) {
        //     starting_vertex
        // } else {
        //     return Err("could not find start vertex".to_string());
        // };
        let mut visiting_vertex = self.find(start).ok_or("could not find start vertex")?;
        let mut visiting_label = *start;
        let mut visiting_score = 0;

        // start at score 0
        state.insert(
            visiting_label,
            VertexState::Explored {
                score: visiting_score,
                coming_from: None,
            },
        );

        // run while there are vertices that are still not completely explored
        while !state
            .get(end)
            .ok_or("could not find end vertex")?
            .is_explored()
        {
            // update all surrounding values according to visiting
            for (adjacent_vertex, weight) in visiting_vertex.borrow().weighted_adjacent_vertices() {
                let adjacent_vertex_label = adjacent_vertex.as_ref().borrow().label();

                if !state
                    .get(&adjacent_vertex_label)
                    .ok_or(format!(
                        "could not find adjacent vertex {}",
                        adjacent_vertex_label
                    ))?
                    .is_explored()
                {
                    state.insert(
                        adjacent_vertex_label,
                        VertexState::Estimated {
                            coming_from: Some(visiting_label),
                            current_score: state[&adjacent_vertex_label]
                                .get_score()
                                .and_then(|curr_score| {
                                    Some(std::cmp::max(curr_score, visiting_score + weight))
                                })
                                .unwrap_or(visiting_score + weight),
                        },
                    );
                }
            }

            // find the lowest unexplored value and set to visiting_label
            if let Some((lowest_label, _score)) = state
                .iter()
                .filter_map(
                    |(label, state)| match (state.is_estimated(), state.get_score()) {
                        (true, Some(score)) => Some((label, score)),
                        _ => None,
                    },
                )
                .sorted_by_key(|set| set.1)
                .nth(0)
            {
                if let Some(lowest_vertex) = self.find(&lowest_label) {
                    visiting_vertex = lowest_vertex;
                } else {
                    return Err(format!(
                        "could not find vertex for lowest label: {}",
                        lowest_label
                    ));
                }

                // update values for next iteration
                visiting_label = visiting_vertex.as_ref().borrow().label();
                visiting_score = state[&visiting_label]
                    .get_score()
                    .ok_or("could not get score of visiting vertex.")?;

                // change the state to explored
                state.insert(
                    visiting_label,
                    VertexState::Explored {
                        score: visiting_score,
                        coming_from: state[&visiting_label].get_path_backwards(),
                    },
                );
            } else if state
                .values()
                .filter(|state| state.is_unexplored())
                .collect::<Vec<&VertexState>>()
                .len()
                > 0
            {
                // if there is nothing estimated, but still some unexplored, we have a loop.
                return Err(format!(
                    "there is no way for getting from {} to {}. There is a loop.",
                    start, end
                ));
            } else {
                // we are done
                return Ok(state);
            }
        }

        Ok(state)
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
