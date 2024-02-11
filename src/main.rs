use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub mod game;

use crate::game::*;
use crate::states::AppState;

mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_plugins(GamePlugin {})
        .run();
}
