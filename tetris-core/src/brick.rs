use std::ops::{Index, IndexMut};

use crate::{
    cell::Cell,
    color::Color,
    traits::{HasSize, IterateDimensions},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Brick([Option<Cell>; 4 * 4]);

const RED: Option<Cell> = Some(Cell::Normal(Color(255, 0, 0)));
const GREEN: Option<Cell> = Some(Cell::Normal(Color(0, 255, 0)));
const BLUE: Option<Cell> = Some(Cell::Normal(Color(0, 0, 255)));
const ORANGE: Option<Cell> = Some(Cell::Normal(Color(255, 165, 0)));
const YELLOW: Option<Cell> = Some(Cell::Normal(Color(255, 255, 0)));
const PURPLE: Option<Cell> = Some(Cell::Normal(Color(255, 0, 255)));
const CYAN: Option<Cell> = Some(Cell::Normal(Color(0, 255, 255)));

#[rustfmt::skip]
impl Brick {
    pub fn z() -> Self {
        Self([
            None, None, None, None,
            RED, RED, None, None,
            None, RED, RED, None,
            None, None, None, None,
        ])
    }

    pub fn s() -> Self {
        Self([
            None, None, None, None,
            None, None, GREEN, GREEN,
            None, GREEN, GREEN, None,
            None, None, None, None,
        ])
    }

    pub fn l() -> Self {
        Self([
            None, ORANGE, None, None,
            None, ORANGE, None, None,
            None, ORANGE, ORANGE, None,
            None, None, None, None,
        ])
    }

    pub fn j() -> Self {
        Self([
            None, None, BLUE, None,
            None, None, BLUE, None,
            None, BLUE, BLUE, None,
            None, None, None, None,
        ])
    }

    pub fn t() -> Self {
        Self([
            None, None, None, None,
            PURPLE, PURPLE, PURPLE, None,
            None, PURPLE, None, None,
            None, None, None, None,
        ])
    }

    pub fn o() -> Self {
        Self([
            None, None, None, None,
            None, YELLOW, YELLOW, None,
            None, YELLOW, YELLOW, None,
            None, None, None, None,
        ])
    }

    pub fn i() -> Self {
        Self([
            None, None, None, None,
            CYAN, CYAN, CYAN, CYAN,
            None, None, None, None,
            None, None, None, None,
        ])
    }

    pub fn by_index(i: i32) -> Self {
        let i = i.abs().wrapping_rem(7);
        match i {
            0 => Self::i(),
            1 => Self::o(),
            2 => Self::t(),
            3 => Self::j(),
            4 => Self::l(),
            5 => Self::s(),
            6 => Self::z(),
            _ => unreachable!(),
        }
    }
}

impl Brick {
    pub fn as_ghost(&self) -> Self {
        let mut new = Self::default();
        for y in 0..4 {
            for x in 0..4 {
                if self.get_item(x, y).is_some() {
                    new[(x, y)] = Some(Cell::Ghost);
                }
            }
        }
        new
    }

    pub fn rotate_left(&self) -> Self {
        let mut new = Self([None; 4 * 4]);
        for i in 0..4 {
            for j in 0..4 {
                new[(i, j)] = self[(4 - j - 1, i)];
            }
        }
        new
    }

    pub fn rotate_right(&self) -> Self {
        self.rotate_left().rotate_left().rotate_left()
    }
}

impl Index<(i32, i32)> for Brick {
    type Output = Option<Cell>;
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        let idx = y * 4 + x;
        &self.0[idx as usize]
    }
}

impl IndexMut<(i32, i32)> for Brick {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        let idx = y * 4 + x;
        &mut self.0[idx as usize]
    }
}

impl HasSize for Brick {
    fn width(&self) -> i32 {
        4
    }

    fn height(&self) -> i32 {
        4
    }
}

impl IterateDimensions for Brick {
    type Output = Option<Cell>;

    fn get_item(&self, x: i32, y: i32) -> Self::Output {
        let idx = y * self.width() + x;
        self.0[idx as usize]
    }
}
