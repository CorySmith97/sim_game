use specs::prelude::*;
use specs_derive::*;
use rltk::console;
use rand::Rng;
use super::{Position, Entity, TileType, Elf};
use std::{thread, time};
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct ElfAI {}

impl<'a> System<'a> for ElfAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Position>,
        Entities<'a>,
        ReadStorage<'a, DesireQueue>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, entities, desires) = data;

        for (entity, pos, desires) in (&entities, &positions, &desires).join() {
            console::log(&format!("Elf is at x {:?}, and y {:?}",  pos.x, pos.y));
            console::log(&format!("Elf wants to {:?}", desires.desires));
        }
    }
}

#[derive(Component)]
pub struct LeftMover {}

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>, 
        WriteStorage<'a, Position>,
        );

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        let mut rng_thread = rand::thread_rng();


        for (_lefty,pos) in (&lefty, &mut pos).join() {
            rng_thread.gen_range(0..3);
            std::thread::sleep(time::Duration::from_millis(1000));
            match rng_thread.gen_range(0..3) {
                0 => pos.x -= 1,
                1 => pos.x += 1,
                2 => pos.y -= 1,
                3 => pos.y += 1,
                _ => {}
            }
        }
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    ((y * 120) + x) as usize
}

fn try_move_elf(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Elf>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(119 , max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));
        }
    }
}

#[derive(Component, Debug)]
pub enum Desire {
    Eat,
    Sleep,
    Drink,
    Work,
}

#[derive(Component, Debug)]
pub struct DesireQueue {
    pub desires: Vec<Desire>,
}

#[derive(Component)]
pub struct DesireSystem {}

impl<'a> System<'a> for DesireSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, DesireQueue>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, mut queue) = data;

        for (pos, queue) in (&mut pos, &mut queue).join() {
            console::log(&format!("Elf is at x {:?}, and y {:?}",  pos.x, pos.y));
            
            for i in queue.desires.iter() {
                console::log(&format!("Elf wants to {:?}", i));
            }
        }
    }
}
