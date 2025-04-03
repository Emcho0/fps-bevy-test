use bevy::prelude::*;

#[derive(Resource,Default)]
pub struct PlayerInput{
    // x komponenta je naprijed
    pub movement: Vec2,

}