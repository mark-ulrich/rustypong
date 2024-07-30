use crate::rect::Rect;

pub trait BoundingBox {
    fn get_bounds(&self) -> Rect;
}