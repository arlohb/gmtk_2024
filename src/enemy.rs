use bevy::prelude::*;

use crate::{Player, Velocity};

#[derive(Component)]
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

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, enemy_movement_system);
}
