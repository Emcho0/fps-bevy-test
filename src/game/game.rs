use bevy::prelude::*;
use window::WindowSettingsPlugin;

use super::window::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowSettingsPlugin);
    }
}
