use bevy::prelude::*;

use crate::{
    elements::ElementInfo,
    molecule::{BuildMolecule, Molecule},
    state::GameState,
    Movement, Velocity,
};

#[derive(Component, Clone)]
pub struct Player;

pub fn create_player(mut cmds: Commands, mut build_molecule_event: EventWriter<BuildMolecule>) {
    let id = cmds
        .spawn((
            SpatialBundle::default(),
            Movement {
                acceleration: 1.,
                max_velocity: 15.,
            },
            Velocity {
                velocity: Vec3::ZERO,
                drag: 0.04,
                max_speed: None,
            },
            Player,
            Molecule {
                elements: vec![ElementInfo::Uranium],
            },
        ))
        .id();

    build_molecule_event.send(BuildMolecule::Create { target: id });
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), create_player);
}
