use bevy::prelude::*;
use rand::Rng;

use crate::{
    elements::ElementInfo,
    enemy::Enemy,
    molecule::{build_molecules_system, BuildMolecule, Molecule},
    state::GameState,
    utils::random_in_donut,
    Player, Velocity,
};

#[derive(Resource, Default)]
pub struct WaveCount(usize);

pub fn reset_wave_count(mut count: ResMut<WaveCount>) {
    count.0 = 0;
}

pub fn wave_check_system(
    enemies: Query<(), With<Enemy>>,
    mut cmds: Commands,
    mut build_molecule_event: EventWriter<BuildMolecule>,
    players: Query<&Transform, With<Player>>,
    mut count: ResMut<WaveCount>,
) {
    if enemies.iter().len() != 0 {
        return;
    }

    count.0 += 1;
    let count = count.0;
    let (enemy_count, types, size_range) = match count {
        count if count <= 3 => (count * 4, vec![ElementInfo::Hydrogen], 1..=1),
        count if count <= 6 => (
            count * 3,
            vec![ElementInfo::Hydrogen, ElementInfo::Uranium],
            1..=2,
        ),
        count if count <= 10 => (
            count * 2,
            vec![
                ElementInfo::Iron,
                ElementInfo::Uranium,
                ElementInfo::Thorium,
            ],
            1..=3,
        ),
        _ => (
            count,
            vec![
                ElementInfo::Iron,
                ElementInfo::Uranium,
                ElementInfo::Thorium,
            ],
            2..=4,
        ),
    };

    let mut rng = rand::thread_rng();
    let Ok(player) = players.get_single() else {
        return;
    };
    let player = player.translation;

    for _ in 0..enemy_count {
        let size = rng.gen_range(size_range.clone());
        let mut elements = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(types[rng.gen_range(0..types.len())]);
        }

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
                Molecule { elements },
                Enemy::new(rng.gen_range(0.2..0.6)),
            ))
            .id();

        build_molecule_event.send(BuildMolecule::Create { target: id });
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<WaveCount>()
        .add_systems(OnEnter(GameState::Playing), reset_wave_count)
        .add_systems(
            Update,
            wave_check_system
                .before(build_molecules_system)
                .run_if(in_state(GameState::Playing)),
        );
}
