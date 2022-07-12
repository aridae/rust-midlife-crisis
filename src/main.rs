//===============================================//
// U r not special                               //
// for winning a game                            //
// with someone who you know was never playing.  //
//===============================================//

// кажется типа как в шарпах
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

// struct Position {
//     x: i32,
//     y: i32,
// }
// 
// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }


// в рамках ECS, указываем что для позишна нужен компонент 
// то же самое, что и выше, спасибо specs-derive
// #[derive(x)] is a macro that says "from my basic data, please derive the boilerplate needed for x"
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    foreground: RGB,
    background: RGB,
}

struct State {
    ecs: World
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, 
                        WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        self.run_systems();
        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // The join function returns an iterator
        // like a database join, it only returns entities that have both
        for (pos, render) in (&positions, &renderables).join() {
            // ctx is the instance of RLTK passed to us when tick runs
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}

#[derive(Component, Debug)]
struct Player {}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("I listen to a lot of True Crime (-.-)")
        .build()?;

    // mut - mutable
    let mut game_state = State {
        ecs: World::new()
    };

    // Specs requires that you register your components at start-up
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<LeftMover>();
    game_state.ecs.register::<Player>();

    // хуяк билдер паттерн
    game_state.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    // хуяк луп вот так делается 
    for i in 0..10 {
        game_state.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }

    println!("Hello, world!"); // println! это макро потому что есть !

    rltk::main_loop(context, game_state)
}


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79 , max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // игрока двигаем
    match ctx.key {
        None => {} // не двигаем...
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}