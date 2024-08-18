use bevy::prelude::*;

use crate::{
    collision::{collision_system, CollisionEvent},
    elements::Atom,
    health::Health,
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
    let Ok(player) = players.get_single() else {
        return;
    };
    let player = player.translation.xy();

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

pub fn enemy_player_collision_system(
    mut events: EventReader<CollisionEvent<Player, Enemy>>,
    players: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&mut Velocity, &Transform), (With<Enemy>, Without<Player>)>,
) {
    // Double enemy - enemy as only one side is pushing here.
    let spacing_factor = 2.0;

    for event in events.read() {
        let Ok(player_trans) = players.get(event.a_id) else {
            return;
        };
        let Ok((mut enemy_vel, enemy_trans)) = enemies.get_mut(event.b_id) else {
            return;
        };

        let difference = (enemy_trans.translation.xy() - player_trans.translation.xy()).normalize();
        let delta = difference * spacing_factor;

        enemy_vel.velocity += Vec3::new(delta.x, delta.y, 0.);
    }
}

pub fn enemy_player_damage_system(
    mut events: EventReader<CollisionEvent<Atom, Atom>>,
    mut healths: Query<&mut Health>,
    parents: Query<&Parent>,
    players: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for event in events.read() {
        let Ok(a_parent) = parents.get(event.a_id) else {
            return;
        };
        let Ok(b_parent) = parents.get(event.b_id) else {
            return;
        };

        let a_is_player = players.contains(a_parent.get());
        let b_is_player = players.contains(b_parent.get());

        if a_is_player && !b_is_player {
            let Ok(mut a_health) = healths.get_mut(event.a_id) else {
                return;
            };

            a_health.health -= 1.;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent<Atom, Atom>>()
        .add_event::<CollisionEvent<Enemy, Enemy>>()
        .add_event::<CollisionEvent<Player, Enemy>>()
        .add_systems(
            FixedUpdate,
            (
                enemy_movement_system,
                collision_system::<Atom, Atom>,
                enemy_player_damage_system,
                collision_system::<Enemy, Enemy>,
                enemy_internal_collision_system,
                collision_system::<Player, Enemy>,
                enemy_player_collision_system,
            )
                .chain(),
        );
}
