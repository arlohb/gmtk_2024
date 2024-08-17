use bevy::prelude::*;

use crate::{
    elements::ElementInfo,
    molecule::{BuildMolecule, Molecule},
    Velocity,
};

#[derive(Component)]
pub struct Enemy;

pub fn create_enemy(mut cmds: Commands, mut build_molecule_event: EventWriter<BuildMolecule>) {
    let id = cmds
        .spawn((
            SpatialBundle::default(),
            Velocity {
                velocity: Vec3::ZERO,
                drag: 0.04,
                max_speed: None,
            },
            Molecule {
                elements: vec![ElementInfo::Uranium],
            },
            Enemy,
        ))
        .id();

    build_molecule_event.send(BuildMolecule(id));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_enemy);
}
