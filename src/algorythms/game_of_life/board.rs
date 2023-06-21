use super::{
    patterns::Patterns,
    pixel::{Pixel, PixelPosition, PixelState},
};
use rand;
use std::{
    cell::RefCell,
    collections::HashMap,
    convert::From,
    hash::Hash,
    ops::{Add, Range, Sub},
    rc::Rc,
};
use terminal_size::{terminal_size, Height, Width};

pub struct BoardDimensions<T> {
    x_max: T,
    x_min: T,
    y_max: T,
    y_min: T,
}

impl<T> BoardDimensions<T>
where
    std::ops::Range<T>: Iterator,
    T: PartialOrd + Clone + From<<Range<T> as Iterator>::Item>,
{
    pub fn new(min: (T, T), max: (T, T)) -> BoardDimensions<T> {
        BoardDimensions {
            x_max: max.0,
            x_min: min.0,
            y_max: max.1,
            y_min: min.1,
        }
    }
    pub fn iter_x(&self) -> Range<T> {
        self.x_min.clone()..self.x_max.clone()
    }
    pub fn iter_y(&self) -> Range<T> {
        self.y_min.clone()..self.y_max.clone()
    }
}

pub struct Board<T> {
    pixels: HashMap<PixelPosition<T>, Rc<RefCell<Pixel<T>>>>,
    pub dimensions: BoardDimensions<T>,
}

impl<T> Board<T>
where
    T: Eq
        + Hash
        + PartialOrd
        + Clone
        + Copy
        + From<<Range<T> as Iterator>::Item>
        + Sub<u8, Output = T>
        + Add<u8, Output = T>,
    Range<T>: Iterator,
    <Range<T> as Iterator>::Item: Copy,
{
    pub fn new(min: (T, T), max: (T, T)) -> Self {
        Board {
            pixels: HashMap::new(),
            dimensions: BoardDimensions::new(min, max),
        }
    }

    pub fn get_pixel(&self, position: (T, T)) -> Option<Rc<RefCell<Pixel<T>>>> {
        match self.pixels.get(&PixelPosition::new(position)) {
            Some(px) => Some(Rc::clone(px)),
            _ => None,
        }
    }

    pub fn fill(&mut self) {
        for x in self.dimensions.iter_x() {
            for y in self.dimensions.iter_y() {
                let position = PixelPosition::<T>::new((x.into(), y.into()));
                self.pixels
                    .entry(position.clone())
                    .or_insert(Rc::new(RefCell::new(Pixel::from(position))));
            }
        }
        self.register_neighbours();
        self.pixels.shrink_to_fit();
    }

    pub fn randomize(&mut self) {
        self.pixels.iter_mut().for_each(|(_pos, pixel)| {
            let mut pixel = pixel.borrow_mut();
            pixel.set_state(rand::random());
            pixel.commit_state()
        })
    }

    pub fn register_neighbours(&mut self) {
        for pixel in self.pixels.values() {
            // let mut pixel = pixel.borrow_mut();
            let position = pixel.borrow().get_postion().clone();

            let mut pixel_positions = vec![];

            let x = position.get_x().clone();
            let y = position.get_y().clone();

            let allow_top = y != self.dimensions.y_min;
            let allow_bottom = y != self.dimensions.y_max;
            let allow_left = x != self.dimensions.x_min;
            let allow_right = x != self.dimensions.x_max;

            if allow_top {
                pixel_positions.push((x, y - 1));
                if allow_left {
                    pixel_positions.push((x - 1, y - 1))
                }
                if allow_right {
                    pixel_positions.push((x + 1, y - 1))
                }
            }
            if allow_bottom {
                pixel_positions.push((x, y + 1));
                if allow_left {
                    pixel_positions.push((x - 1, y + 1))
                }
                if allow_right {
                    pixel_positions.push((x + 1, y + 1))
                }
            }

            if allow_left {
                pixel_positions.push((x - 1, y))
            }
            if allow_right {
                pixel_positions.push((x + 1, y))
            }

            for (x, y) in pixel_positions {
                if let Some(neighbour) = self.get_pixel((x, y)) {
                    let nbv = Rc::clone(&neighbour);
                    pixel.borrow_mut().register_neighbour(nbv)
                }
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut has_movement = false;
        for pixel in self.pixels.values_mut() {
            let mut pixel = pixel.borrow_mut();
            // println!("x:{}, y:{}, alive_neighbours: {}", pixel.get_postion().get_x(), pixel.get_postion().get_y(), pixel.count_alive_neighbours())
            let new_state = match (pixel.count_alive_neighbours(), pixel.get_state()) {
                (3, _) => PixelState::Alive,
                (2, curr) => curr.clone(),
                (_, _) => PixelState::Dead,
            };
            if &new_state != pixel.get_state() && !has_movement {
                has_movement = true
            }
            pixel.set_state(new_state);
        }
        for pixel in self.pixels.values_mut() {
            pixel.borrow_mut().commit_state();
        }
        has_movement
    }

    pub fn has_live(&self) -> bool {
        self.pixels.values().fold(false, |has_live, pixel| {
            if pixel.borrow().get_state() == &PixelState::Alive {
                true
            } else {
                has_live
            }
        })
    }

    pub fn count_pixels(&self) -> usize {
        self.pixels.values().count()
    }
}

impl Board<u8> {
    pub fn fmt(&self) -> String {
        let mut rows = vec![];
        let (max_width, max_height) =
            terminal_size().unwrap_or((Width(u16::MAX), Height(u16::MAX)));

        for y in self.dimensions.iter_y().step_by(2) {
            if y as u16 > max_height.0 * 2 - 1 {
                break;
            }
            let mut row = String::new();
            for x in self.dimensions.iter_x() {
                if Width(x as u16) > max_width {
                    break;
                }

                row.push(
                    match (
                        self.get_pixel((x, y))
                            .and_then(|px| Some(px.borrow().get_state().clone()))
                            .unwrap_or(PixelState::default()),
                        self.get_pixel((x, y + 1))
                            .and_then(|px| Some(px.borrow().get_state().clone()))
                            .unwrap_or(PixelState::default()),
                    ) {
                        (PixelState::Alive, PixelState::Alive) => '█',
                        (PixelState::Alive, PixelState::Dead) => '▀',
                        (PixelState::Dead, PixelState::Alive) => '▄',
                        (PixelState::Dead, PixelState::Dead) => ' ',
                    },
                )
            }
            rows.push(row);
        }
        rows.join("\n")
    }
    pub fn create_life(&mut self, pattern: Patterns) {
        for (x, y) in pattern.get_life_positions() {
            let mut pixel = self
                .pixels
                .entry(PixelPosition::new((x, y)))
                .or_insert(Rc::new(RefCell::new(Pixel::new((x, y)))))
                .borrow_mut();
            pixel.set_state(PixelState::Alive);
            pixel.commit_state();
        }
    }
}

impl Default for Board<u8> {
    fn default() -> Self {
        Board {
            pixels: HashMap::new(),
            dimensions: BoardDimensions::new((u8::MIN, u8::MIN), (u8::MAX, u8::MAX)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::patterns::Patterns;
    use super::Board;
    use clearscreen;
    #[test]
    fn create_borad() {
        let mut board = Board::<u8>::default();
        board.fill();
        board.randomize();
        assert_eq!(board.count_pixels(), u8::MAX as usize * u8::MAX as usize);
        while board.has_live() {
            let _ = clearscreen::clear();
            println!("{}", board.fmt());
            // std::thread::sleep(std::time::Duration::from_millis(100));
            if !board.tick() {
                break;
            }
        }
    }
    #[test]
    fn create_copperhead() {
        let mut board = Board::new((0, 0), (100, 20));
        board.fill();
        board.create_life(Patterns::Copperhead);
        while board.has_live() {
            let _ = clearscreen::clear();
            println!("{}", board.fmt());
            std::thread::sleep(std::time::Duration::from_millis(100));
            if !board.tick() {
                break;
            }
        }
    }

    #[test]
    fn create_snarkloop() {
        let mut board = Board::new((0, 0), (100, 100));
        board.fill();
        board.create_life(Patterns::SnarkLoop);
        while board.has_live() {
            let _ = clearscreen::clear();
            println!("{}", board.fmt());
            std::thread::sleep(std::time::Duration::from_millis(100));
            if !board.tick() {
                break;
            }
        }
    }
}
