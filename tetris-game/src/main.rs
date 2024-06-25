use area::Area;
use audio_box::{ROTATE_SOUND_BYTES, WRONG_MOVE_SOUND_BYTES};
use clap::Parser;
use config::Config;
use tetris_core::{
    cell::Cell,
    prelude::{Color as TetrisColor, Tetris},
    traits::{HasSize, IterateDimensions, Randomizer},
};

mod area;
mod audio_box;
mod config;

use raylib::prelude::*;

pub static BRICK_IMAGE: &[u8] = include_bytes!("../brick.png");

#[derive(Debug, Clone, Default)]
struct DummyRng(i32);

impl Randomizer for DummyRng {
    fn new() -> Self {
        Self(1)
    }

    fn with_seed(seed: i32) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> i32 {
        let v = self.0.wrapping_mul(123);
        self.0 = v;
        v
    }
}

pub fn main() {
    dotenvy::dotenv().ok();
    let config = Config::parse();
    let mut tetris = Tetris::new(
        config.width as i32,
        config.height as i32,
        DummyRng::with_seed(rand::random()),
    );
    let (mut rl, thread) = raylib::init()
        .size(920, 720)
        .title("Tetris")
        .resizable()
        .build();

    let brick_image = Image::load_image_from_mem(".png", BRICK_IMAGE).unwrap();
    let brick_texture = rl.load_texture_from_image(&thread, &brick_image).unwrap();

    // TODO: let mut audio = AudioBox::new();
    // audio.load_sound_from_bytes("rotate", ROTATE_SOUND_BYTES, ".ogg");
    // audio.load_sound_from_bytes("rotate", ROTATE_SOUND_BYTES, ".ogg");
    // audio.load_sound_from_bytes("wrong", WRONG_MOVE_SOUND_BYTES, ".ogg");

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
        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
        handle_events(&mut tetris, &mut rl, &rotate_sound, &wrong_move_sound);

        let cell_size = resize_playfield(
            (width, height),
            tetris.width(),
            tetris.height(),
            &mut playfield,
        );

        tetris.tick();

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::new(0, 44, 88, 255));
        draw_playfield(&playfield, &tetris, &mut draw, cell_size, &brick_texture);

        draw_boxed(
            (playfield.x() / 2.0 - cell_size * 2.0, cell_size * 2.0),
            cell_size,
            Some(tetris.next()),
            "Next",
            &mut draw,
            &brick_texture,
        );

        draw_boxed(
            (playfield.x() / 2.0 - cell_size * 2.0, cell_size * 8.0),
            cell_size,
            tetris.held(),
            "Hold",
            &mut draw,
            &brick_texture,
        );

        draw_score(cell_size, tetris.score(), &mut draw, &playfield);
    }
}

fn draw_score(cell_size: f32, score: u32, draw: &mut RaylibDrawHandle, playfield_area: &Area) {
    let text = format!("{:0>5}", score);
    let text_w = draw.measure_text(&text, cell_size as i32);
    draw.draw_text(
        &text,
        (playfield_area.x() / 2.0) as i32 - text_w / 2,
        8,
        cell_size as i32,
        Color::WHITE,
    );
}

fn draw_boxed(
    (ox, oy): (f32, f32),
    cell_size: f32,
    item: Option<impl IterateDimensions<Output = Option<Cell>>>,
    label: &str,
    draw: &mut RaylibDrawHandle,
    brick_texture: &Texture2D,
) {
    draw.draw_text(
        label,
        (ox + cell_size) as i32,
        oy as i32,
        cell_size as i32,
        Color::WHITE,
    );
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
                            brick_texture,
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
                    brick_texture,
                );
            }
        });
    } else {
        draw_rect(
            draw,
            (ox, oy, (cell_size * 4.0).ceil(), (cell_size * 4.0).ceil()),
            Color::BLACK,
            brick_texture,
        );
    }
}

fn handle_events(
    tetris: &mut Tetris<DummyRng>,
    rl: &mut RaylibHandle,
    rotate_sound: &Sound,
    wrong_sound: &Sound,
) {
    let pressed_key = rl.get_key_pressed();
    if let Some(key) = pressed_key {
        match key {
            KeyboardKey::KEY_LEFT => {
                if tetris.move_left() {
                } else {
                    wrong_sound.play();
                }
            }
            KeyboardKey::KEY_RIGHT => {
                if tetris.move_right() {
                } else {
                    wrong_sound.play();
                }
            }
            KeyboardKey::KEY_DOWN => {
                tetris.move_down();
            }
            KeyboardKey::KEY_A => {
                if tetris.rotate_left() {
                    rotate_sound.play();
                } else {
                    wrong_sound.play();
                }
            }
            KeyboardKey::KEY_D => {
                if tetris.rotate_right() {
                    rotate_sound.play();
                } else {
                    wrong_sound.play();
                }
            }
            KeyboardKey::KEY_SPACE => {
                tetris.drop_block();
            }
            KeyboardKey::KEY_S => {
                tetris.swap_held();
            }
            KeyboardKey::KEY_R => {
                let t = Tetris::new(
                    tetris.width(),
                    tetris.height(),
                    DummyRng::with_seed(tetris.score() as i32),
                );
                *tetris = t;
            }
            _ => {}
        }
    }
}

fn draw_playfield(
    playfield_area: &Area,
    tetris: &Tetris<DummyRng>,
    draw: &mut RaylibDrawHandle,
    cell_size: f32,
    brick_texture: &Texture2D,
) {
    tetris.iter_dim(|x, y, c| {
        stroke_rect(
            draw,
            (
                (x as f32 * cell_size + playfield_area.x()).ceil(),
                (y as f32 * cell_size + playfield_area.y()).ceil(),
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
                            (x as f32 * cell_size + playfield_area.x()).ceil(),
                            (y as f32 * cell_size + playfield_area.y()).ceil(),
                            cell_size.ceil(),
                            cell_size.ceil(),
                        ),
                        Color::new(r, g, b, 255),
                        brick_texture,
                    );
                }
                tetris_core::cell::Cell::Ghost => {
                    draw_rect(
                        draw,
                        (
                            (x as f32 * cell_size + playfield_area.x()).ceil(),
                            (y as f32 * cell_size + playfield_area.y()).ceil(),
                            cell_size.ceil(),
                            cell_size.ceil(),
                        ),
                        Color::new(255, 255, 255, 127),
                        brick_texture,
                    );
                }
            }
        }
    });
}

fn draw_rect(
    draw: &mut RaylibDrawHandle,
    (x, y, w, h): (f32, f32, f32, f32),
    color: Color,
    brick_texture: &Texture2D,
) {
    draw.draw_texture_pro(
        brick_texture,
        Rectangle::new(
            0.0,
            0.0,
            brick_texture.width as f32,
            brick_texture.height as f32,
        ),
        Rectangle::new(x, y, w, h),
        Vector2::zero(),
        0.0,
        color,
    );
}

fn stroke_rect(draw: &mut RaylibDrawHandle, (x, y, w, h): (f32, f32, f32, f32), color: Color) {
    draw.draw_rectangle_lines(x as i32, y as i32, w as i32, h as i32, color);
}

fn resize_playfield((w, h): (i32, i32), tetris_w: i32, tetris_h: i32, pf: &mut Area) -> f32 {
    let cell_size = h as f32 / tetris_h as f32;
    let pf_width = cell_size * tetris_w as f32;
    *pf = Area::new(w as f32 / 2.0 - pf_width / 2.0, 0.0, cell_size, cell_size);
    cell_size
}
