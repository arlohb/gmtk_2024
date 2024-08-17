use bevy::prelude::*;

use crate::{
    elements::{BuildElements, ElementInfo},
    molecule::Molecule,
    Movement, Velocity,
};

#[derive(Component)]
pub struct Player;

pub fn create_player(mut cmds: Commands, build_elements: Res<BuildElements>) {
    cmds.spawn((
        SpatialBundle::default(),
        Movement {
            acceleration: 1.,
            max_velocity: 15.,
        },
        Velocity {
            velocity: Vec3::ZERO,
            drag: 0.04,
        },
        Player,
        Molecule {
            elements: vec![ElementInfo::Uranium],
        },
    ));
    cmds.run_system(build_elements.0);
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_player);
}
