use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    // pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
    //     Rect{x1: x, y1: y, x2: x+w, y2: y+h}
    // }
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect{x1: x, y1: y, x2: x+w, y2: y+h}
    }
    pub fn intersect(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
        (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        (x > self.x1) && (x < self.x2) && (y > self.y1) && (y < self.y2)
    }
}
