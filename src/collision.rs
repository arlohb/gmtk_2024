use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent<A: Component + Clone, B: Component + Clone> {
    pub a_id: Entity,
    pub a_comp: A,
    pub b_id: Entity,
    pub b_comp: B,
}

pub fn circle_collision(a_center: Vec2, a_radius: f32, b_center: Vec2, b_radius: f32) -> bool {
    (a_center - b_center).length() - a_radius - b_radius <= 0.
}

pub fn collision_system<A: Component + Clone, B: Component + Clone>(
    query_a: Query<(Entity, &GlobalTransform, &Sprite, &A), Without<B>>,
    query_b: Query<(Entity, &GlobalTransform, &Sprite, &B), Without<A>>,
    mut event_writer: EventWriter<CollisionEvent<A, B>>,
) {
    for (a_id, a_transform, a_sprite, a_comp) in &query_a {
        let a_center = a_transform.translation().xy();
        let Some(a_radius) = a_sprite.custom_size.map(|custom_size| custom_size.x) else {
            continue;
        };

        for (b_id, b_transform, b_sprite, b_comp) in &query_b {
            let b_center = b_transform.translation().xy();
            let Some(b_radius) = b_sprite.custom_size.map(|custom_size| custom_size.x) else {
                continue;
            };

            if circle_collision(a_center, a_radius, b_center, b_radius) {
                event_writer.send(CollisionEvent {
                    a_id,
                    a_comp: a_comp.clone(),
                    b_id,
                    b_comp: b_comp.clone(),
                });
            }
        }
    }
}
