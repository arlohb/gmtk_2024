use bevy::prelude::*;
use rand::Rng;

use crate::{
    elements::ElementInfo,
    enemy::Enemy,
    molecule::{build_molecules_system, BuildMolecule, Molecule},
    Velocity,
};

pub fn wave_check_system(
    enemies: Query<(), With<Enemy>>,
    mut cmds: Commands,
    mut build_molecule_event: EventWriter<BuildMolecule>,
) {
    if enemies.iter().len() != 0 {
        return;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let id = cmds
            .spawn((
                SpatialBundle::from_transform(Transform::from_xyz(
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(-1000.0..1000.0),
                    0.,
                )),
                Velocity {
                    velocity: Vec3::ZERO,
                    drag: rng.gen_range(0.025..0.055),
                    max_speed: Some(rng.gen_range(30.0..70.0)),
                },
                Molecule {
                    elements: vec![ElementInfo::Uranium],
                },
                Enemy {
                    speed: rng.gen_range(0.2..0.6),
                },
            ))
            .id();

        build_molecule_event.send(BuildMolecule::Create { target: id });
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, wave_check_system.before(build_molecules_system));
}
