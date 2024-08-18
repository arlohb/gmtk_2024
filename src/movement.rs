use bevy::prelude::*;

use crate::Velocity;

#[derive(Component)]
pub struct Movement {
    pub acceleration: f32,
    pub max_velocity: f32,
}

pub fn movement_system(
    mut query: Query<(&mut Velocity, &Movement)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, movement)) = query.get_single_mut() else {
        return;
    };

    let mut offset = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        offset += Vec2::new(0., movement.acceleration);
    }

    if keys.pressed(KeyCode::KeyS) {
        offset += Vec2::new(0., -movement.acceleration);
    }

    if keys.pressed(KeyCode::KeyD) {
        offset += Vec2::new(movement.acceleration, 0.);
    }

    if keys.pressed(KeyCode::KeyA) {
        offset += Vec2::new(-movement.acceleration, 0.);
    }

    offset = offset.clamp_length_max(movement.acceleration);

    velocity.velocity += Vec3::new(offset.x, offset.y, 0.);
    velocity.velocity = velocity.velocity.clamp_length_max(movement.max_velocity);
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, movement_system);
}
