use bevy::{ecs::system::SystemId, prelude::*};
use std::time::Duration;

use crate::{MainCamera, Player, TimeToLive, Velocity};

pub fn create_bullet(
    In((origin, target)): In<(Vec2, Vec2)>,
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
    ));
}

#[derive(Resource)]
pub struct CreateBullet(SystemId<(Vec2, Vec2)>);

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
        cmds.run_system_with_input(create_bullet.0, (origin, target));
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<CreateBullet>()
        .add_systems(Update, player_shoot);
}
