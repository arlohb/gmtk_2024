use bevy::{ecs::system::SystemId, prelude::*};
use std::time::Duration;

use crate::{
    collision::{collision_system, CollisionEvent},
    elements::Atom,
    health::Health,
    MainCamera, Player, TimeToLive, Velocity,
};

#[derive(Component, Clone, PartialEq, Eq)]
pub enum Bullet {
    FromPlayer,
    FromEnemy,
}

pub fn create_bullet(
    In((origin, dir, bullet)): In<(Vec2, Vec2, Bullet)>,
    mut cmds: Commands,
    assets: ResMut<AssetServer>,
) {
    let speed = 25.;
    // Just in case the caller didn't normalise it
    let dir = dir.normalize();

    cmds.spawn((
        Velocity {
            velocity: Vec3::new(dir.x, dir.y, 0.) * speed,
            drag: 0.,
            max_speed: None,
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(28., 28.)),
                ..Default::default()
            },
            texture: assets.load("Electron.png"),
            transform: Transform::from_xyz(origin.x, origin.y, 1.),
            ..Default::default()
        },
        TimeToLive::new(Duration::from_secs(2)),
        bullet,
    ));
}

#[derive(Resource)]
pub struct CreateBullet(SystemId<(Vec2, Vec2, Bullet)>);

impl FromWorld for CreateBullet {
    fn from_world(world: &mut World) -> Self {
        Self(world.register_system(create_bullet))
    }
}

#[derive(Component)]
pub struct Shooter;

fn player_shoot(
    create_bullet: Res<CreateBullet>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    shooters: Query<(&GlobalTransform, &Parent), With<Shooter>>,
    players: Query<&Transform, With<Player>>,
    mouse_btns: Res<ButtonInput<MouseButton>>,
    mut cmds: Commands,
) {
    let (camera, camera_transform) = cameras.single();
    let window = windows.single();
    let Some(target) = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
    else {
        return;
    };

    if mouse_btns.just_pressed(MouseButton::Left) {
        for (shooter, parent) in &shooters {
            let Ok(player) = players.get(parent.get()) else {
                continue;
            };

            let origin = shooter.translation().xy();
            let dir = target - player.translation.xy();

            cmds.run_system_with_input(create_bullet.0, (origin, dir, Bullet::FromPlayer));
        }
    }
}

pub fn bullet_hit_system(
    mut events: EventReader<CollisionEvent<Bullet, Atom>>,
    players: Query<Entity, With<Player>>,
    mut healths: Query<(&mut Health, &Parent), With<Atom>>,
    mut cmds: Commands,
) {
    let damage = 100. / 8.;

    for CollisionEvent {
        a_id: bullet,
        a_comp: bullet_type,
        b_id: atom,
        ..
    } in events.read()
    {
        let Ok((mut health, parent)) = healths.get_mut(*atom) else {
            return;
        };

        let is_player_atom = players.contains(**parent);

        let is_hit = is_player_atom && *bullet_type == Bullet::FromEnemy
            || !is_player_atom && *bullet_type == Bullet::FromPlayer;

        if is_hit {
            health.health -= damage;
            let _ = cmds.get_entity(*bullet).map(|mut entity| entity.despawn());
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<CreateBullet>()
        .add_event::<CollisionEvent<Bullet, Atom>>()
        .add_systems(Update, player_shoot)
        .add_systems(
            Update,
            (collision_system::<Bullet, Atom>, bullet_hit_system).chain(),
        );
}
