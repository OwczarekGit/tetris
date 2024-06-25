use crate::{
    board::Board,
    brick::Brick,
    cell::Cell,
    player::Player,
    traits::{HasSize, IterateDimensions, Randomizer},
};

const EXTRA_FRAMES: u32 = 2;

#[derive(Debug, Default, Clone)]
pub struct Tetris<R> {
    board: Board,
    player: Player,
    ghost: Player,
    next_player: Brick,
    held: Option<Brick>,
    step_timer: u32,
    score: u32,
    randomizer: R,
}

impl<R: Randomizer> Tetris<R> {
    pub fn new(w: i32, h: i32, mut randomizer: R) -> Self {
        Self {
            board: Board::new(w, h),
            player: Player::with_brick_centered_rand(w, randomizer.next()),
            next_player: Brick::by_index(randomizer.next()),
            step_timer: 1,
            randomizer,
            ..Default::default()
        }
    }
    pub fn move_left(&mut self) -> bool {
        let moved = self.player.move_left();
        if moved.brick_fits(&self.board) {
            self.player = moved;
            self.step_timer = self.step_timer.saturating_sub(EXTRA_FRAMES).max(1);
            true
        } else {
            false
        }
    }
    pub fn move_right(&mut self) -> bool {
        let moved = self.player.move_right();
        if moved.brick_fits(&self.board) {
            self.player = moved;
            self.step_timer = self.step_timer.saturating_sub(EXTRA_FRAMES).max(1);
            true
        } else {
            false
        }
    }
    pub fn move_down(&mut self) -> bool {
        let moved = self.player.move_down();
        if moved.brick_fits(&self.board) {
            self.player = moved;
            self.step_timer = 1;
            self.score += 1;
            true
        } else {
            self.board
                .insert_brick(self.player.position(), self.player.brick());
            self.player = Player::with_brick_centered(self.next_player, self.width());
            self.next_player = self.random_brick();
            self.score += 2;
            false
        }
    }
    pub fn rotate_left(&mut self) -> bool {
        let moved = self.player.rotate_left();
        if moved.brick_fits(&self.board) {
            self.player = moved;
            self.step_timer = self.step_timer.saturating_sub(EXTRA_FRAMES).max(1);
            true
        } else {
            false
        }
    }
    pub fn rotate_right(&mut self) -> bool {
        let moved = self.player.rotate_right();
        if moved.brick_fits(&self.board) {
            self.player = moved;
            self.step_timer = self.step_timer.saturating_sub(EXTRA_FRAMES).max(1);
            true
        } else {
            false
        }
    }

    fn random_brick(&mut self) -> Brick {
        Brick::by_index(self.randomizer.next())
    }

    pub fn drop_block(&mut self) {
        let mut dropped = self.player;
        'dropped: loop {
            let lower = dropped.move_down();
            if lower.brick_fits(&self.board) {
                dropped = lower;
            } else {
                break 'dropped;
            }
        }
        self.board.insert_brick(dropped.position(), dropped.brick());
        self.player = Player::with_brick_centered(self.next_player, self.width());
        self.next_player = self.random_brick();
        self.step_timer = 1;
        self.score += 1;
    }

    pub fn swap_held(&mut self) -> bool {
        if let Some(h) = self.held {
            let changed = self.player.set_brick(h);
            if self.board.brick_fits(changed.position(), changed.brick()) {
                let current = self.player.brick();
                self.held = Some(current);
                self.player = changed;
                return true;
            }
        } else {
            self.held = Some(self.player.brick());
            self.player = Player::with_brick_centered(self.next_player, self.width());
            self.next_player = self.random_brick();
            self.step_timer -= self.step_timer.saturating_sub(EXTRA_FRAMES).max(1);
            return true;
        }
        false
    }

    pub fn tick(&mut self) {
        if self.step_timer % 60 == 0 {
            self.move_down();
        }

        let removed = self.board.clean_drop();
        self.score += removed * removed.pow(3);

        let mut ghost = self.player;
        'ghost: loop {
            let lower = ghost.move_down().as_ghost();
            if lower.brick_fits(&self.board) {
                ghost = lower;
            } else {
                break 'ghost;
            }
        }

        self.ghost = ghost;
        self.step_timer += 1;
    }

    pub fn next(&self) -> Brick {
        self.next_player
    }

    pub fn held(&self) -> Option<Brick> {
        self.held
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}

impl<R> HasSize for Tetris<R> {
    fn width(&self) -> i32 {
        self.board.width()
    }

    fn height(&self) -> i32 {
        self.board.height()
    }
}

impl<R> IterateDimensions for Tetris<R> {
    type Output = Option<Cell>;

    fn get_item(&self, x: i32, y: i32) -> Self::Output {
        self.board.get_item(x, y)
    }

    fn iter_dim(&self, mut action: impl FnMut(i32, i32, Self::Output)) {
        let mut board = self.board.clone();
        board.insert_brick(self.ghost.position(), self.ghost.brick());
        board.insert_brick(self.player.position(), self.player.brick());

        for y in 0..self.height() {
            for x in 0..self.width() {
                action(x, y, board.get_item(x, y));
            }
        }
    }
}
