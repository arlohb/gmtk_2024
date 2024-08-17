use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    // TODO: Make Vec2
    pub velocity: Vec3,
    pub drag: f32,
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation += velocity.velocity;

        let v_decrease = velocity.velocity * velocity.drag;
        velocity.velocity -= v_decrease;
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedPostUpdate, apply_velocity);
}
