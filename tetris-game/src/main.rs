use area::Area;
use tetris_core::{
    cell::Cell,
    prelude::{Color as TetrisColor, Tetris},
    traits::{HasSize, IterateDimensions},
};

mod area;

use raylib::prelude::*;

const PLAYFIELD_PADDING: i32 = 0;
const ROTATE_SOUND_BYTES: &[u8] = include_bytes!("../rotate.ogg");
const WRONG_MOVE_SOUND_BYTES: &[u8] = include_bytes!("../wrong_move.ogg");

pub fn main() {
    dotenvy::dotenv().ok();
    let mut tetris = Tetris::new(10, 20, rand::random());
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Tetris")
        .resizable()
        .build();

    let audio = RaylibAudio::init_audio_device().unwrap();
    let rotate_sound = audio
        .new_wave_from_memory(".ogg", ROTATE_SOUND_BYTES)
        .unwrap();
    let wrong_move_sound = audio
        .new_wave_from_memory(".ogg", WRONG_MOVE_SOUND_BYTES)
        .unwrap();

    let rotate_sound = audio.new_sound_from_wave(&rotate_sound).unwrap();
    let wrong_move_sound = audio.new_sound_from_wave(&wrong_move_sound).unwrap();

    let mut playfield = Area::default();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        handle_events(&mut tetris, &mut rl, &rotate_sound, &wrong_move_sound);

        let cell_size = resize_playfield(
            rl.get_screen_height(),
            tetris.width(),
            tetris.height(),
            &mut playfield,
        );

        tetris.tick();

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::new(0, 44, 88, 255));
        draw_playfield(&tetris, &mut draw, cell_size);

        draw_boxed(
            (playfield.width() as f32 + cell_size, cell_size * 2.0),
            cell_size,
            Some(tetris.next()),
            "Next",
            &mut draw,
        );

        draw_boxed(
            (playfield.width() as f32 + cell_size, cell_size * 8.0),
            cell_size,
            tetris.held(),
            "Hold",
            &mut draw,
        );

        draw_score(
            (playfield.width() as f32 + cell_size, cell_size / 2.0),
            tetris.score(),
            &mut draw,
        );
    }
}

fn draw_score((ox, oy): (f32, f32), score: u32, draw: &mut RaylibDrawHandle) {
    let text = format!("{:0>12}", score);
    draw.draw_text(&text, ox as i32, oy as i32, 48, Color::WHITE);
}

fn draw_boxed(
    (ox, oy): (f32, f32),
    cell_size: f32,
    item: Option<impl IterateDimensions<Output = Option<Cell>>>,
    label: &str,
    draw: &mut RaylibDrawHandle,
) {
    draw.draw_text(label, (ox + cell_size) as i32, oy as i32, 48, Color::WHITE);
    let oy = oy + cell_size;

    if let Some(item) = item {
        item.iter_dim(|x, y, c| {
            if let Some(c) = c {
                match c {
                    Cell::Normal(TetrisColor(r, g, b)) => {
                        draw_rect(
                            draw,
                            (
                                (x as f32 * cell_size + ox).ceil(),
                                (y as f32 * cell_size + oy).ceil(),
                                cell_size.ceil(),
                                cell_size.ceil(),
                            ),
                            Color::new(r, g, b, 255),
                        );
                    }
                    Cell::Ghost => todo!(),
                }
            } else {
                draw_rect(
                    draw,
                    (
                        (x as f32 * cell_size + ox).ceil(),
                        (y as f32 * cell_size + oy).ceil(),
                        cell_size.ceil(),
                        cell_size.ceil(),
                    ),
                    Color::BLACK,
                );
            }
        });
    } else {
        draw_rect(
            draw,
            (ox, oy, (cell_size * 4.0).ceil(), (cell_size * 4.0).ceil()),
            Color::BLACK,
        );
    }
}

fn handle_events(
    tetris: &mut Tetris,
    rl: &mut RaylibHandle,
    rotate_sound: &Sound<'_>,
    wrong_move_sound: &Sound<'_>,
) {
    let pressed_key = rl.get_key_pressed();
    if let Some(key) = pressed_key {
        match key {
            KeyboardKey::KEY_LEFT => {
                if tetris.move_left() {
                } else {
                    wrong_move_sound.play();
                }
            }
            KeyboardKey::KEY_RIGHT => {
                if tetris.move_right() {
                } else {
                    wrong_move_sound.play();
                }
            }
            KeyboardKey::KEY_DOWN => {
                tetris.move_down();
            }
            KeyboardKey::KEY_A => {
                if tetris.rotate_left() {
                    rotate_sound.play();
                } else {
                    wrong_move_sound.play();
                }
            }
            KeyboardKey::KEY_D => {
                if tetris.rotate_right() {
                    rotate_sound.play();
                } else {
                    wrong_move_sound.play();
                }
            }
            KeyboardKey::KEY_SPACE => {
                tetris.drop_block();
            }
            KeyboardKey::KEY_S => {
                tetris.swap_held();
            }
            _ => {}
        }
    }
}

fn draw_playfield(tetris: &Tetris, draw: &mut RaylibDrawHandle, cell_size: f32) {
    tetris.iter_dim(|x, y, c| {
        stroke_rect(
            draw,
            (
                (x as f32 * cell_size).ceil(),
                (y as f32 * cell_size).ceil(),
                cell_size.ceil(),
                cell_size.ceil(),
            ),
            Color::new(255, 255, 255, 100),
        );
        if let Some(c) = c {
            match c {
                tetris_core::cell::Cell::Normal(TetrisColor(r, g, b)) => {
                    draw_rect(
                        draw,
                        (
                            (x as f32 * cell_size).ceil(),
                            (y as f32 * cell_size).ceil(),
                            cell_size.ceil(),
                            cell_size.ceil(),
                        ),
                        Color::new(r, g, b, 255),
                    );
                }
                tetris_core::cell::Cell::Ghost => {
                    draw_rect(
                        draw,
                        (
                            (x as f32 * cell_size).ceil(),
                            (y as f32 * cell_size).ceil(),
                            cell_size.ceil(),
                            cell_size.ceil(),
                        ),
                        Color::new(255, 255, 255, 127),
                    );
                }
            }
        }
    });
}

fn draw_rect(draw: &mut RaylibDrawHandle, (x, y, w, h): (f32, f32, f32, f32), color: Color) {
    draw.draw_rectangle(x as i32, y as i32, w as i32, h as i32, color);
}

fn stroke_rect(draw: &mut RaylibDrawHandle, (x, y, w, h): (f32, f32, f32, f32), color: Color) {
    draw.draw_rectangle_lines(x as i32, y as i32, w as i32, h as i32, color);
}

fn resize_playfield(h: i32, tetris_w: i32, tetris_h: i32, pf: &mut Area) -> f32 {
    let cell_size = h as f32 / tetris_h as f32;
    *pf = Area::new(
        PLAYFIELD_PADDING as f32,
        PLAYFIELD_PADDING as f32,
        (cell_size * tetris_w as f32) - PLAYFIELD_PADDING as f32,
        (cell_size * tetris_h as f32) - PLAYFIELD_PADDING as f32,
    );
    cell_size
}
