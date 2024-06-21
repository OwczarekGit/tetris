pub trait HasSize {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn size(&self) -> (i32, i32) {
        (self.width(), self.height())
    }
}

pub trait IterateDimensions: HasSize {
    type Output;
    fn get_item(&self, x: i32, y: i32) -> Self::Output;
    fn iter_dim(&self, mut action: impl FnMut(i32, i32, Self::Output)) {
        let (w, h) = self.size();

        for y in 0..h {
            for x in 0..w {
                action(x, y, self.get_item(x, y));
            }
        }
    }
}
