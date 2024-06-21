use std::ops::{Index, IndexMut};

use crate::traits::{HasSize, IterateDimensions};

#[derive(Debug, Default, Clone, Copy)]
pub struct Brick([bool; 4 * 4]);

impl Brick {
    pub fn z() -> Self {
        Self([
            false, false, false, false, true, true, false, false, false, true, true, false, false,
            false, false, false,
        ])
    }

    pub fn s() -> Self {
        Self([
            false, false, false, false, false, false, true, true, false, true, true, false, false,
            false, false, false,
        ])
    }

    pub fn l() -> Self {
        Self([
            false, true, false, false, false, true, false, false, false, true, true, false, false,
            false, false, false,
        ])
    }

    pub fn j() -> Self {
        Self([
            false, false, true, false, false, false, true, false, false, true, true, false, false,
            false, false, false,
        ])
    }

    pub fn t() -> Self {
        Self([
            true, true, true, false, false, true, false, false, false, false, false, false, false,
            false, false, false,
        ])
    }

    pub fn o() -> Self {
        Self([
            false, false, false, false, false, true, true, false, false, true, true, false, false,
            false, false, false,
        ])
    }

    pub fn i() -> Self {
        Self([
            false, false, false, false, true, true, true, true, false, false, false, false, false,
            false, false, false,
        ])
    }

    pub fn by_index(i: i32) -> Self {
        let i = i % 7;
        match i {
            0 => Self::i(),
            1 => Self::o(),
            2 => Self::t(),
            3 => Self::j(),
            4 => Self::l(),
            5 => Self::s(),
            _ => Self::z(),
        }
    }
}

impl Brick {
    pub fn rotate_left(&self) -> Self {
        let mut new = Self([false; 4 * 4]);
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
    type Output = bool;
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
    type Output = bool;

    fn get_item(&self, x: i32, y: i32) -> Self::Output {
        let idx = y * self.width() + x;
        self.0[idx as usize]
    }
}
