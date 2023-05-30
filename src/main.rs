use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{max, min};
use rand::Rng;

mod components;
pub use components::*;
mod elves;
pub use elves::*;
mod maps;
pub use maps::*;
mod rect;
pub use rect::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    ecs: World,
    pub runstate: RunState,
}


impl State {
    fn run_systems(&mut self) {
        let mut elf_ai = ElfAI {};
        elf_ai.run_now(&self.ecs);
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        let mut desires = DesireSystem {};
        desires.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();


        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = RunState::Running;
        }


        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

         for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    let context = Rltk::init_simple8x8(120, 80, "Fantasy SIm", "resources");
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<ElfAI>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Desire>();
    gs.ecs.register::<DesireSystem>();
    gs.ecs.register::<DesireQueue>();


    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 50, y: 15 })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::AZURE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(ElfAI {})
        .with(LeftMover {})
        .with(DesireSystem {})
        .build();
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('f'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Desire::Work)
        .build();

    rltk::main_loop(context, gs)
}
