use bevy::prelude::*;

pub fn follow_system<
    // Marker component on the entity to being moved, can be multiple entities.
    Follower: Component,
    // Marker component on the entity being followed, can only be one entity.
    Target: Component,
    // Ratio from 0-100 between the current position and target position each tick.
    const T: usize,
>(
    mut followers: Query<&mut Transform, (With<Follower>, Without<Target>)>,
    target: Query<&Transform, With<Target>>,
) {
    let Ok(target) = target.get_single() else {
        return;
    };
    let target = target.translation;

    for mut follower in &mut followers {
        let z = follower.translation.z;
        follower.translation = follower.translation.lerp(target, T as f32 / 100.);
        follower.translation.z = z;
    }
}
