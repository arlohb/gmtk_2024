mod movement;
pub use movement::*;
mod velocity;
pub use velocity::*;
mod camera;
pub use camera::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_player, setup_camera))
        .add_systems(FixedUpdate, movement_system)
        .add_systems(
            FixedPostUpdate,
            (apply_velocity, follow_player.after(apply_velocity)),
        )
        .run();
}
