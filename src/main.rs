mod map;

// Make own prelude to simplify module access
// b/c top-level of crate, don't need to make public - mods branching from crate are visible throughout program
mod prelude {
    pub use bracket_lib::prelude::*; // publicly using re-exports inside this prelude; anything that uses this prelude, also uses bracket_lib
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*; // use prelude above to make it available to the main scope in main.rs

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
