use crate::rect::Rect;

pub trait BoxCollidable {
    fn get_bounds(&self) -> Rect;
    fn collides_with(&self, other: &dyn BoxCollidable) -> bool;
    fn on_collision(&mut self, other: &dyn BoxCollidable);
    fn get_tags(&self) -> &Vec<String>;
}
