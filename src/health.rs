use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self { health }
    }
}
