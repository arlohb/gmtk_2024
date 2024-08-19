use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    elements::ElementInfo,
    enemy::Enemy,
    molecule::{build_molecules_system, BuildMolecule, Molecule},
    utils::random_in_donut,
    Player, Velocity,
};

pub fn wave_check_system(
    enemies: Query<(), With<Enemy>>,
    mut cmds: Commands,
    mut build_molecule_event: EventWriter<BuildMolecule>,
    players: Query<&Transform, With<Player>>,
) {
    if enemies.iter().len() != 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player = players.single().translation;

    for _ in 0..20 {
        let id = cmds
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(
                    (player.xy() + random_in_donut(2000., 6000.)).extend(0.),
                )),
                Velocity {
                    velocity: Vec3::ZERO,
                    drag: rng.gen_range(0.025..0.055),
                    max_speed: Some(rng.gen_range(15.0..20.0)),
                },
                Molecule {
                    elements: vec![ElementInfo::Uranium],
                },
                Enemy::new(
                    rng.gen_range(0.2..0.6),
                    Duration::from_secs_f32(rng.gen_range(1.5..2.0)),
                ),
            ))
            .id();

        build_molecule_event.send(BuildMolecule::Create { target: id });
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, wave_check_system.before(build_molecules_system));
}
