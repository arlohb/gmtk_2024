mod movement;
pub use movement::*;
mod velocity;
pub use velocity::*;

use bevy::prelude::*;

fn create_player(mut cmds: Commands, assets: ResMut<AssetServer>) {
    let circle = assets.load("Circle.png");

    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::linear_rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            texture: circle,
            ..Default::default()
        },
        Movement {
            acceleration: 1.,
            max_velocity: 15.,
        },
        Velocity {
            velocity: Vec3::ZERO,
            drag: 0.04,
        },
    ));
}

fn create_camera(mut cmds: Commands, mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(Color::BLACK);

    cmds.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..Default::default()
        },
        projection: OrthographicProjection {
            scale: 1.,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 10.),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_player, create_camera))
        .add_systems(FixedUpdate, movement_system)
        .add_systems(FixedPostUpdate, apply_velocity)
        .run();
}
