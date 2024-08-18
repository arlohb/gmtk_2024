use bevy::{
    core_pipeline::bloom::{BloomCompositeMode, BloomSettings},
    prelude::*,
};

use crate::{follow::follow_system, molecule::Molecule, velocity, Player};

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
        BloomSettings {
            intensity: 0.3,
            ..BloomSettings::NATURAL
        },
        MainCamera,
    ));
}

pub fn zoom_camera(
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    player: Query<&Molecule, With<Player>>,
) {
    let mut camera = camera.single_mut();
    let Ok(player) = player.get_single() else {
        return;
    };

    let target = player.elements.len() as f32 / 8. + 1.;
    camera.scale = camera.scale.lerp(target, 0.1);
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera)
        .add_systems(
            FixedPostUpdate,
            follow_system::<MainCamera, Player, 10>.after(velocity::apply_velocity),
        )
        .add_systems(Update, zoom_camera);
}
