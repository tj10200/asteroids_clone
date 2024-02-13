use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;

pub mod components;
mod game;
mod mainmenu;
mod states;
mod systems;

use crate::game::*;
use crate::mainmenu::MainMenuPlugin;
use crate::states::AppState;
use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins(MainMenuPlugin {})
        .add_plugins(GamePlugin {})
        .run();
}
