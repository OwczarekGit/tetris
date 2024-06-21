use cell::Cell;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::FontStyle;
use sdl2::video::Window;
use tetris::Tetris;
use traits::IterateDimensions;

mod board;
mod brick;
mod cell;
mod color;
mod player;
mod tetris;
mod traits;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};
use std::time::Duration;

const SCALE: i32 = 28;

pub fn main() {
    dotenvy::dotenv().ok();
    let font_bytes = include_bytes!("../PixelFJVerdana12pt.ttf");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let rwops = RWops::from_bytes(font_bytes).unwrap();
    let mut font = ttf.load_font_from_rwops(rwops, 16).unwrap();
    font.set_style(FontStyle::BOLD);

    let mut tetris = Tetris::new(10, 20, 1231);

    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            if handle_events(&mut tetris, event) {
                break 'running;
            }
        }

        tetris.iter_dim(|x, y, c| {
            let rect = Rect::new(x * SCALE, y * SCALE, SCALE as u32, SCALE as u32);
            canvas.set_draw_color(Color::RGB(44, 44, 44));
            let _ = canvas.draw_rect(rect);
            if let Some(c) = c {
                let color = match c {
                    Cell::Normal(color) => color.into(),
                    Cell::Ghost => pixels::Color::RGB(255, 255, 255),
                };
                canvas.set_draw_color(color);
                let _ = canvas.fill_rect(rect);
            }
        });

        if let Some(held) = tetris.held() {
            draw_brick(&mut canvas, (12, 10), held);
        }

        draw_brick(&mut canvas, (12, 4), tetris.next());
        let surface = font
            .render(&format!("{:0>12}", tetris.score()))
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(16 * SCALE, 0 * SCALE, width, height);
        let _ = canvas.copy(&texture, None, Some(target));

        canvas.present();
        tetris.tick();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn handle_events(tetris: &mut Tetris, ev: Event) -> bool {
    match ev {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            return true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            tetris.swap_held();
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            tetris.rotate_left();
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            tetris.rotate_right();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            tetris.move_down();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            tetris.move_left();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            tetris.move_right();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            tetris.drop_block();
        }
        _ => {}
    }
    false
}

fn draw_brick(
    canvas: &mut Canvas<Window>,
    (ox, oy): (i32, i32),
    brick: impl IterateDimensions<Output = Option<Cell>>,
) {
    brick.iter_dim(|x, y, c| {
        if let Some(Cell::Normal(color)) = c {
            let rect = sdl2::rect::Rect::new(
                (x + ox) * SCALE,
                (y + oy) * SCALE,
                SCALE as u32,
                SCALE as u32,
            );
            canvas.set_draw_color(sdl2::pixels::Color::from(color));
            let _ = canvas.fill_rect(rect);
        }
        if let Some(Cell::Ghost) = c {
            let rect = sdl2::rect::Rect::new(
                (x + ox) * SCALE,
                (y + oy) * SCALE,
                SCALE as u32,
                SCALE as u32,
            );
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = canvas.fill_rect(rect);
        }
    });
}

impl From<crate::color::Color> for sdl2::pixels::Color {
    fn from(crate::color::Color(r, g, b): crate::color::Color) -> Self {
        Self { r, g, b, a: 255 }
    }
}
