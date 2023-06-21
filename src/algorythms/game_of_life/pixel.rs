use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; // 0.8.0

use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PixelState {
    Alive,
    #[default]
    Dead,
}

impl Distribution<PixelState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PixelState {
        match rng.gen_bool(0.5) {
            true => PixelState::Alive,
            false => PixelState::Dead,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PixelPosition<T> {
    x: T,
    y: T,
}

impl<T> PixelPosition<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new(position: (T, T)) -> PixelPosition<T> {
        PixelPosition {
            x: position.0,
            y: position.1,
        }
    }
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
pub struct Pixel<T> {
    position: PixelPosition<T>,
    state: PixelState,
    next_state: Option<PixelState>,
    neighbours: HashMap<PixelPosition<T>, Rc<RefCell<Pixel<T>>>>,
}

impl<T> Pixel<T>
where
    T: PartialEq + Eq + Hash + Clone + Copy,
{
    pub fn new(position: (T, T)) -> Self {
        Pixel {
            position: PixelPosition::new(position),
            state: Default::default(),
            next_state: None,
            neighbours: HashMap::with_capacity(8),
        }
    }
    pub fn get_state(&self) -> &PixelState {
        &self.state
    }
    pub fn get_postion(&self) -> &PixelPosition<T> {
        &self.position
    }
    pub fn set_state(&mut self, state: PixelState) {
        self.next_state = Some(state);
    }
    pub fn commit_state(&mut self) {
        match self.next_state {
            Some(state) => self.state = state,
            _ => {}
        }
    }
    pub fn register_neighbour(&mut self, neighbour: Rc<RefCell<Pixel<T>>>) {
        let position = neighbour.borrow().get_postion().clone();
        self.neighbours.entry(position).or_insert(neighbour);
    }
    pub fn count_alive_neighbours(&self) -> u8 {
        let v = self.neighbours.values();
        v.fold(0, |acc, nb| match nb.borrow().get_state() {
            PixelState::Alive => acc + 1,
            _ => acc,
        })
    }
}

impl<T> From<PixelPosition<T>> for Pixel<T>
where
    T: PartialEq + Eq + Hash,
{
    fn from(position: PixelPosition<T>) -> Self {
        Pixel {
            position,
            state: Default::default(),
            next_state: None,
            neighbours: HashMap::with_capacity(8),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Pixel, PixelPosition, PixelState};
    use std::collections::HashMap;
    #[test]
    fn generate_pixel() {
        let pixel = Pixel {
            position: PixelPosition { x: 2, y: 8 },
            state: Default::default(),
            next_state: None,
            neighbours: HashMap::with_capacity(8),
        };
        assert_eq!(pixel.get_postion().get_x(), &2);
        assert_eq!(pixel.get_postion().get_y(), &8);
        assert_eq!(pixel.get_state(), &PixelState::default());
    }
}
