
mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use std::vec;

use prelude::*;


struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self{
        Self { map: Map::new(), player: Player::new(Point { x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2})}
    }
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => ctx.quitting = true,
                _ => {},
            }
        }

        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}



fn main() -> BError{

    let arr = vec![1, 2, 3];

    use prelude::*;

    let ctx = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Roguelike")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
} 
