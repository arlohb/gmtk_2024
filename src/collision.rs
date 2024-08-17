use bevy::prelude::*;

pub trait CollisionEvent<A: Component, B: Component>: Event {
    fn from_collision(a_id: Entity, a_comp: &A, b_id: Entity, b_comp: &B) -> Self;
}

pub fn circle_collision(a_center: Vec2, a_radius: f32, b_center: Vec2, b_radius: f32) -> bool {
    (a_center - b_center).length() - a_radius - b_radius <= 0.
}

pub fn collision_system<A: Component, B: Component, E: CollisionEvent<A, B>>(
    query_a: Query<(Entity, &GlobalTransform, &Sprite, &A), Without<B>>,
    query_b: Query<(Entity, &GlobalTransform, &Sprite, &B), Without<A>>,
    mut event_writer: EventWriter<E>,
) {
    for (a, a_transform, a_sprite, a_comp) in &query_a {
        let a_center = a_transform.translation().xy();
        let Some(a_radius) = a_sprite.custom_size.map(|custom_size| custom_size.x) else {
            continue;
        };

        for (b, b_transform, b_sprite, b_comp) in &query_b {
            let b_center = b_transform.translation().xy();
            let Some(b_radius) = b_sprite.custom_size.map(|custom_size| custom_size.x) else {
                continue;
            };

            if circle_collision(a_center, a_radius, b_center, b_radius) {
                event_writer.send(E::from_collision(a, a_comp, b, b_comp));
            }
        }
    }
}
