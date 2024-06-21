use board::Board;
use brick::Brick;
use player::Player;
use sdl2::render::Canvas;
use sdl2::video::Window;
use traits::{HasSize, IterateDimensions};

mod board;
mod brick;
mod player;
mod traits;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const SCALE: i32 = 28;

fn random_player(width: i32) -> Player {
    let i: i32 = rand::random();
    Player::with_brick_centered_rand(width, i)
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut board = Board::default();
    let mut player = random_player(board.width());
    let mut next_player = random_player(board.width());
    let mut hold: Option<Brick> = None;

    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut drop_timer = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if let Some(h) = hold {
                        let changed = player.set_brick(h);
                        if board.brick_fits(changed.position(), changed.brick()) {
                            let current = player.brick();
                            hold = Some(current);
                            player = changed;
                        }
                    } else {
                        hold = Some(player.brick());
                        player = next_player;
                        next_player = random_player(board.width());
                        drop_timer -= 30;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let moved = player.rotate_left();
                    if moved.brick_fits(&board) {
                        player = moved;
                        drop_timer = -30;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let moved = player.rotate_right();
                    if moved.brick_fits(&board) {
                        player = moved;
                        drop_timer = -30;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let moved = player.move_down();
                    if moved.brick_fits(&board) {
                        player = moved;
                        drop_timer = 1;
                    } else {
                        board.insert_brick(player.position(), player.brick());
                        player = random_player(board.width());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let moved = player.move_left();
                    if moved.brick_fits(&board) {
                        player = moved;
                        drop_timer -= 30;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let moved = player.move_right();
                    if moved.brick_fits(&board) {
                        player = moved;
                        drop_timer -= 30;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let mut dropped = player;
                    'dropped: loop {
                        let lower = dropped.move_down();
                        if lower.brick_fits(&board) {
                            dropped = lower;
                        } else {
                            break 'dropped;
                        }
                    }
                    board.insert_brick(dropped.position(), dropped.brick());
                    player = next_player;
                    next_player = random_player(board.width());
                }
                _ => {}
            }
        }

        if drop_timer % 60 == 0 {
            let moved = player.move_down();
            if moved.brick_fits(&board) {
                player = moved;
            } else {
                board.insert_brick(player.position(), player.brick());
                player = next_player;
                next_player = random_player(board.width());
            }
        }

        board.clean_drop();

        board.iter_dim(|x, y, c| {
            let rect = sdl2::rect::Rect::new(x * SCALE, y * SCALE, SCALE as u32, SCALE as u32);
            canvas.set_draw_color(Color::RGB(44, 44, 44));
            let _ = canvas.draw_rect(rect);
            if c {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                let _ = canvas.fill_rect(rect);
            }
        });

        draw_brick(
            &mut canvas,
            player.position(),
            player.brick(),
            Color::RGB(255, 0, 0),
        );
        draw_brick(
            &mut canvas,
            (12, 2),
            next_player.brick(),
            Color::RGB(255, 0, 0),
        );

        let mut ghost = player;
        'ghost: loop {
            let lower = ghost.move_down();
            if lower.brick_fits(&board) {
                ghost = lower;
            } else {
                break 'ghost;
            }
        }

        draw_brick(
            &mut canvas,
            ghost.position(),
            ghost.brick(),
            Color::RGB(200, 200, 200),
        );

        if let Some(h) = hold {
            draw_brick(&mut canvas, (12, 6), h, Color::RGB(255, 0, 0));
        }

        canvas.present();
        drop_timer += 1;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_brick(
    canvas: &mut Canvas<Window>,
    (ox, oy): (i32, i32),
    brick: impl IterateDimensions<Output = bool>,
    color: Color,
) {
    brick.iter_dim(|x, y, c| {
        if c {
            let rect = sdl2::rect::Rect::new(
                (x + ox) * SCALE,
                (y + oy) * SCALE,
                SCALE as u32,
                SCALE as u32,
            );
            canvas.set_draw_color(color);
            let _ = canvas.fill_rect(rect);
        }
    });
}
