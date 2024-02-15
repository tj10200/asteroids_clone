use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;

pub mod components;
mod game;
mod game_over_menu;
mod mainmenu;
mod states;
mod systems;

use crate::game::*;
use crate::game_over_menu::GameOverPlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::states::AppState;
use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins((MainMenuPlugin {}, GameOverPlugin {}))
        .add_plugins(GamePlugin {})
        .run();
}
