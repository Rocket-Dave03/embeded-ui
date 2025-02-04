use rgb::RGBA8;

use crate::{Coord, Size};

pub trait Element: Drawable {
    fn size(&self) -> Size;
}

pub trait Drawable {
    fn get_pixel(&self, _coord: Coord) -> RGBA8;
}
