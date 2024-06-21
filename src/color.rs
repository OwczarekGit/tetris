use sdl2::pixels;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Color> for pixels::Color {
    fn from(Color(r, g, b): Color) -> Self {
        Self { r, g, b, a: 255 }
    }
}
