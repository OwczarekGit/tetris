use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::FontStyle;
use sdl2::video::Window;
use tetris_core::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use std::time::Duration;

const SCALE: i32 = 28;

pub fn main() {
    dotenvy::dotenv().ok();
    let font_bytes = include_bytes!("../PixelFJVerdana12pt.ttf");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let rwops = RWops::from_bytes(font_bytes).unwrap();
    let mut font = ttf.load_font_from_rwops(rwops, 16).unwrap();
    font.set_style(FontStyle::BOLD);

    let available = controller_subsystem.num_joysticks().unwrap();

    let _controller = (0..available).find_map(|id| {
        if !controller_subsystem.is_game_controller(id) {
            return None;
        }

        match controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                Some(c)
            }
            Err(_) => None,
        }
    });

    let mut tetris = Tetris::new(10, 20, 1231);

    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(SdlColor::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(SdlColor::RGB(0, 33, 44));
        canvas.clear();
        for event in event_pump.poll_iter() {
            if handle_events(&mut tetris, event) {
                break 'running;
            }
        }

        tetris.iter_dim(|x, y, c| {
            let rect = Rect::new(x * SCALE, y * SCALE, SCALE as u32, SCALE as u32);
            canvas.set_draw_color(SdlColor::RGB(44, 44, 44));
            let _ = canvas.draw_rect(rect);
            if let Some(c) = c {
                let color = match c {
                    Cell::Normal(color) => SdlColor::RGB(color.0, color.1, color.2),
                    Cell::Ghost => SdlColor::RGBA(255, 255, 255, 20),
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
            .blended(SdlColor::RGB(255, 255, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(16 * SCALE, 0, width, height);
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
        Event::ControllerButtonDown { button, .. } => match button {
            sdl2::controller::Button::A => {
                tetris.drop_block();
            }
            sdl2::controller::Button::X => {
                tetris.swap_held();
            }
            sdl2::controller::Button::LeftShoulder => {
                tetris.rotate_left();
            }
            sdl2::controller::Button::RightShoulder => {
                tetris.rotate_right();
            }
            sdl2::controller::Button::DPadDown => {
                tetris.move_down();
            }
            sdl2::controller::Button::DPadLeft => {
                tetris.move_left();
            }
            sdl2::controller::Button::DPadRight => {
                tetris.move_right();
            }
            _ => {}
        },
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
            canvas.set_draw_color(SdlColor::RGB(color.0, color.1, color.2));
            let _ = canvas.fill_rect(rect);
        }
        if let Some(Cell::Ghost) = c {
            let rect = sdl2::rect::Rect::new(
                (x + ox) * SCALE,
                (y + oy) * SCALE,
                SCALE as u32,
                SCALE as u32,
            );
            canvas.set_draw_color(SdlColor::RGB(255, 255, 255));
            let _ = canvas.fill_rect(rect);
        }
    });
}
