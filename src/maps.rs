use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB, Rltk, RandomNumberGenerator};

#[derive(PartialEq, Copy, Clone, Component)]
pub enum TileType { Wall, Floor }

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 120) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 120*80];
    for x in 0..120 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 79)] = TileType::Wall;
    }
    for y in 0..80 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(119, y)] = TileType::Wall;
    }
   let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..6320 {
        let x = rng.roll_dice(1, 119);
        let y = rng.roll_dice(1, 79);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) { 
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => { ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.')); 
            }
            TileType::Wall => { ctx.set(x, y, RGB::from_f32(0., 1.0, 0.), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#')); 
            }
        }
        x += 1;
        if x > 119 {
            x = 0;
            y += 1;
        }
    }
}

