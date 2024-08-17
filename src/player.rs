use bevy::prelude::*;

use crate::{
    elements::{build_elements, ElementInfo},
    Movement, Velocity,
};

#[derive(Component)]
pub struct Player {
    pub elements: Vec<ElementInfo>,
}

pub fn create_player(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        SpatialBundle::default(),
        Movement {
            acceleration: 1.,
            max_velocity: 15.,
        },
        Velocity {
            velocity: Vec3::ZERO,
            drag: 0.04,
        },
        Player {
            elements: vec![ElementInfo::Uranium],
        },
    ))
    .with_children(|parent| {
        for child in build_elements(&[ElementInfo::Uranium], &assets) {
            parent.spawn(child);
        }
    });
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_player);
}
