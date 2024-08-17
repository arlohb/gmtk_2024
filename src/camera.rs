use bevy::prelude::*;

use crate::{follow::follow_system, velocity, Player};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut cmds: Commands, mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(Color::BLACK);

    cmds.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            projection: OrthographicProjection {
                scale: 1.5,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        },
        MainCamera,
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera).add_systems(
        FixedPostUpdate,
        follow_system::<MainCamera, Player, 10>.after(velocity::apply_velocity),
    );
}
