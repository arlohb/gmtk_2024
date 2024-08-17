use bevy::prelude::*;

use crate::{
    elements::{BuildElements, ElementInfo},
    Movement, Velocity,
};

#[derive(Component)]
pub struct Player {
    pub elements: Vec<ElementInfo>,
}

pub fn create_player(mut cmds: Commands, build_elements: Res<BuildElements>) {
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
    ));
    cmds.run_system_with_input(build_elements.0, vec![ElementInfo::Uranium]);
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_player);
}
