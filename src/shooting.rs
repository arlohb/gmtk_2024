use bevy::{ecs::system::SystemId, prelude::*};
use std::time::Duration;

use crate::{
    collision::{collision_system, CollisionEvent},
    elements::Atom,
    health::Health,
    MainCamera, Player, TimeToLive, Velocity,
};

#[derive(Component, Clone)]
pub enum Bullet {
    FromPlayer,
    FromEnemy,
}

pub fn create_bullet(
    In((origin, target, bullet)): In<(Vec2, Vec2, Bullet)>,
    mut cmds: Commands,
    assets: ResMut<AssetServer>,
) {
    let speed = 25.;
    let dir = (target - origin).normalize();

    cmds.spawn((
        Velocity {
            velocity: Vec3::new(dir.x, dir.y, 0.) * speed,
            drag: 0.,
            max_speed: None,
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16., 16.)),
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

fn player_shoot(
    create_bullet: Res<CreateBullet>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    players: Query<&GlobalTransform, With<Player>>,
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

    let player = players.single();
    let origin = player.translation().xy();

    if mouse_btns.just_pressed(MouseButton::Left) {
        cmds.run_system_with_input(create_bullet.0, (origin, target, Bullet::FromPlayer));
    }
}

// TODO, Need to despawn the bullet
#[derive(Event)]
pub struct BulletHit(pub Entity, pub Bullet);

impl CollisionEvent<Bullet, Atom> for BulletHit {
    fn from_collision(_: Entity, bullet: &Bullet, b: Entity, _: &Atom) -> Self {
        Self(b, bullet.clone())
    }
}

pub fn bullet_hit_system(
    mut events: EventReader<BulletHit>,
    players: Query<Entity, With<Player>>,
    mut healths: Query<(&mut Health, &Parent), With<Atom>>,
) {
    let damage = 100. / 8.;

    for BulletHit(atom, bullet) in events.read() {
        let Ok((mut health, parent)) = healths.get_mut(*atom) else {
            return;
        };

        let is_player_atom = players.contains(**parent);

        match (is_player_atom, bullet) {
            (true, Bullet::FromEnemy) => health.health -= damage,
            (false, Bullet::FromPlayer) => health.health -= damage,
            _ => (),
        };
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<CreateBullet>()
        .add_event::<BulletHit>()
        .add_systems(Update, player_shoot)
        .add_systems(
            Update,
            (
                collision_system::<Bullet, Atom, BulletHit>,
                bullet_hit_system,
            )
                .chain(),
        );
}
