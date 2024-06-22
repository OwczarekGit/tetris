use crate::{
    board::Board,
    brick::Brick,
    cell::Cell,
    traits::{HasSize, IterateDimensions},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    position: (i32, i32),
    brick: Brick,
}

impl Player {
    pub fn as_ghost(&self) -> Self {
        Self {
            brick: self.brick.as_ghost(),
            ..*self
        }
    }

    pub fn with_brick_centered(brick: Brick, width: i32) -> Self {
        let x = width / 2 - brick.width() / 2;
        Self {
            position: (x, 0),
            brick,
        }
    }

    pub fn with_brick_centered_rand(width: i32, idx: i32) -> Self {
        let brick = Brick::by_index(idx);
        Self::with_brick_centered(brick, width)
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn rotate_left(&self) -> Self {
        Self {
            position: self.position,
            brick: self.brick.rotate_left(),
        }
    }

    pub fn rotate_right(&self) -> Self {
        Self {
            position: self.position,
            brick: self.brick.rotate_right(),
        }
    }

    pub fn move_down(&self) -> Self {
        Self {
            position: (self.position.0, self.position.1 + 1),
            ..*self
        }
    }

    pub fn move_left(&self) -> Self {
        Self {
            position: (self.position.0 - 1, self.position.1),
            ..*self
        }
    }

    pub fn move_right(&self) -> Self {
        Self {
            position: (self.position.0 + 1, self.position.1),
            ..*self
        }
    }

    pub fn brick_fits(&self, board: &Board) -> bool {
        board.brick_fits(self.position, self.brick)
    }

    pub fn brick(&self) -> Brick {
        self.brick
    }

    pub fn set_brick(&self, brick: Brick) -> Self {
        Self { brick, ..*self }
    }
}

impl HasSize for Player {
    fn width(&self) -> i32 {
        self.brick.width()
    }

    fn height(&self) -> i32 {
        self.brick.height()
    }
}

impl IterateDimensions for Player {
    type Output = Option<Cell>;

    fn get_item(&self, x: i32, y: i32) -> Self::Output {
        self.brick.get_item(x, y)
    }
}
