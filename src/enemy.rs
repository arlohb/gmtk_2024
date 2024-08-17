use bevy::prelude::*;

use crate::{
    collision::{collision_system, CollisionEvent},
    Player, Velocity,
};

#[derive(Component, Clone)]
pub struct Enemy {
    pub speed: f32,
}

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Velocity, &Transform, &Enemy)>,
    players: Query<&Transform, With<Player>>,
) {
    let player = players.single().translation.xy();

    for (mut velocity, origin, &Enemy { speed }) in &mut enemies {
        let offset = player - origin.translation.xy();

        if offset.length() <= 0.0001 {
            continue;
        }

        let dir = offset.normalize();
        let accel = speed * dir;

        velocity.velocity += Vec3::new(accel.x, accel.y, 0.);
    }
}

pub fn enemy_internal_collision_system(
    mut events: EventReader<CollisionEvent<Enemy, Enemy>>,
    mut transforms: Query<(&mut Velocity, &Transform), With<Enemy>>,
) {
    let spacing_factor = 0.5;

    for event in events.read() {
        let Ok([(mut a_vel, a_trans), (mut b_vel, b_trans)]) =
            transforms.get_many_mut([event.a_id, event.b_id])
        else {
            continue;
        };

        let a = a_trans.translation.xy();
        let b = b_trans.translation.xy();

        let difference = (a - b).normalize();
        let a_delta = difference * spacing_factor;
        let b_delta = -difference * spacing_factor;

        a_vel.velocity += Vec3::new(a_delta.x, a_delta.y, 0.);
        b_vel.velocity += Vec3::new(b_delta.x, b_delta.y, 0.);
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent<Enemy, Enemy>>()
        .add_systems(FixedUpdate, enemy_movement_system)
        .add_systems(
            FixedUpdate,
            (
                collision_system::<Enemy, Enemy>,
                enemy_internal_collision_system,
            )
                .chain(),
        );
}
