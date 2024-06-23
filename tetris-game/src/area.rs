use tetris_core::traits::HasSize;

#[derive(Debug, Default, Clone, Copy)]
pub struct Area {
    position: (f32, f32),
    size: (f32, f32),
}

impl Area {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: (x, y),
            size: (w, h),
        }
    }
}

impl Area {
    pub fn x(&self) -> f32 {
        self.position.0
    }
    pub fn y(&self) -> f32 {
        self.position.1
    }
    pub fn x_end(&self) -> f32 {
        self.position.0 + self.size.0
    }
    pub fn y_end(&self) -> f32 {
        self.position.0 + self.size.1
    }
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x(), self.y(), self.x_end(), self.y_end())
    }
}

impl HasSize for Area {
    fn width(&self) -> i32 {
        (self.size.0 - self.position.0) as i32
    }

    fn height(&self) -> i32 {
        (self.size.1 - self.position.1) as i32
    }
}
