use bevy::prelude::*;

use crate::molecule::Molecule;

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
    query_a: Query<(
        Entity,
        &GlobalTransform,
        Option<&Sprite>,
        Option<&Molecule>,
        &A,
    )>,
    query_b: Query<(
        Entity,
        &GlobalTransform,
        Option<&Sprite>,
        Option<&Molecule>,
        &B,
    )>,
    mut event_writer: EventWriter<CollisionEvent<A, B>>,
) {
    for (a_id, a_transform, a_sprite, a_molecule, a_comp) in &query_a {
        let a_center = a_transform.translation().xy();

        let a_radius = match (a_molecule, a_sprite) {
            (Some(molecule), _) => molecule.collision_radius(),
            (
                _,
                Some(Sprite {
                    custom_size: Some(size),
                    ..
                }),
            ) => size.x,
            _ => return,
        };

        for (b_id, b_transform, b_sprite, b_molecule, b_comp) in &query_b {
            let b_center = b_transform.translation().xy();

            let b_radius = match (b_molecule, b_sprite) {
                (Some(molecule), _) => molecule.collision_radius(),
                (
                    _,
                    Some(Sprite {
                        custom_size: Some(size),
                        ..
                    }),
                ) => size.x,
                _ => return,
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
