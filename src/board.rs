use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{
    brick::Brick,
    traits::{HasSize, IterateDimensions},
};

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 20;

#[derive(Debug, Clone, Copy)]
pub struct Board([bool; WIDTH as usize * HEIGHT as usize]);

impl Board {
    pub fn insert_brick(&mut self, (ox, oy): (i32, i32), brick: Brick) {
        brick.iter_dim(|x, y, c| {
            if c {
                self.set_field((ox + x, oy + y), true);
            }
        });
    }

    pub fn set_field(&mut self, (x, y): (i32, i32), state: bool) {
        if x >= 0 && x < self.width() && y >= 0 && y < self.height() {
            self[(x, y)] = state;
        }
    }

    pub fn is_taken(&self, (x, y): (i32, i32)) -> bool {
        if x >= 0 && x < self.width() && y >= 0 && y < self.height() {
            self[(x, y)]
        } else {
            true
        }
    }

    pub fn brick_fits(&self, (ox, oy): (i32, i32), brick: Brick) -> bool {
        let mut res = true;
        brick.iter_dim(|x, y, c| {
            if c && self.is_taken((x + ox, y + oy)) {
                res = false;
            }
        });
        res
    }

    pub fn line_full(&self, y: i32) -> bool {
        if y > self.height() - 1 {
            return false;
        }
        for x in 0..self.width() {
            if !self.is_taken((x, y)) {
                return false;
            }
        }
        true
    }

    pub fn clear_line(&mut self, y: i32) {
        for x in 0..self.width() {
            self.set_field((x, y), false);
        }
    }

    pub fn drop_line(&mut self, y: i32) {
        if y < self.height() - 1 {
            for x in 0..self.width() {
                let tmp = self[(x, y)];
                self[(x, y)] = false;
                self[(x, y + 1)] = tmp;
            }
        }
    }

    pub fn clean_drop(&mut self) {
        for y in (0..self.height()).rev() {
            if self.line_full(y) {
                self.clear_line(y);
                for dy in (0..y).rev() {
                    self.drop_line(dy);
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self([false; WIDTH as usize * HEIGHT as usize])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.width();
        self.iter_dim(|x, _, c| {
            if c {
                let _ = write!(f, "X");
            } else {
                let _ = write!(f, ".");
            }
            if x == w - 1 {
                let _ = writeln!(f);
            }
        });
        Ok(())
    }
}

impl HasSize for Board {
    fn width(&self) -> i32 {
        WIDTH as i32
    }

    fn height(&self) -> i32 {
        HEIGHT as i32
    }
}

impl IterateDimensions for Board {
    type Output = bool;

    fn get_item(&self, x: i32, y: i32) -> Self::Output {
        self[(x, y)]
    }
}

impl Index<(i32, i32)> for Board {
    type Output = bool;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        let idx = y * self.width() + x;
        &self.0[idx as usize]
    }
}

impl IndexMut<(i32, i32)> for Board {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        let idx = y * self.width() + x;
        &mut self.0[idx as usize]
    }
}
