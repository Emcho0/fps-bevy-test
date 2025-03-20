use bevy::prelude::*;

use super::{level::level, player::player, window::window};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level::LevelPlugin,
            player::PlayerPlugin,
            window::WindowSettingsPlugin,
        ));
    }
}
