use bevy::prelude::*;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
    .add_plugins(DefaultPlugins).run();
}

