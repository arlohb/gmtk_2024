use bevy::prelude::*;

use crate::{
    elements::ElementInfo,
    molecule::{BuildMolecule, Molecule},
    Player, Velocity,
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
                max_speed: Some(50.),
            },
            Molecule {
                elements: vec![ElementInfo::Uranium],
            },
            Enemy,
        ))
        .id();

    build_molecule_event.send(BuildMolecule::Create { target: id });
}

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
    players: Query<&Transform, With<Player>>,
) {
    let speed = 0.4;

    let player = players.single().translation.xy();

    for (mut velocity, origin) in &mut enemies {
        let offset = player - origin.translation.xy();

        if offset.length() <= 0.0001 {
            continue;
        }

        let dir = offset.normalize();
        let accel = speed * dir;

        velocity.velocity += Vec3::new(accel.x, accel.y, 0.);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_enemy)
        .add_systems(FixedUpdate, enemy_movement_system);
}
