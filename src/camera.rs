use bevy::prelude::*;

use crate::{velocity, Player};

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
                scale: 1.,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        },
        MainCamera,
    ));
}

pub fn follow_player(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let camera = &mut camera.single_mut().translation;
    let player = player.single().translation;

    let z = camera.z;
    *camera = camera.lerp(player, 0.1);
    camera.z = z;
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera).add_systems(
        FixedPostUpdate,
        follow_player.after(velocity::apply_velocity),
    );
}
