use bevy::prelude::*;

use crate::{Movement, Velocity};

#[derive(Component)]
pub struct Player;

pub fn create_player(mut cmds: Commands, assets: Res<AssetServer>) {
    let circle = assets.load("ElementU.png");

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
        Player,
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_player);
}
