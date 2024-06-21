use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Normal(Color),
    Ghost,
}
