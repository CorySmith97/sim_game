use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Hand {
    pub entity: Entity,
    pub manipulation: i32,
}

#[derive(Component)]
pub struct Elf {
}
