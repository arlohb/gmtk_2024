use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

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

#[derive(Component)]
pub struct Cursor;

const CURSOR_SIZE: f32 = 186.;

pub fn setup_cursor(mut cmds: Commands, assets: Res<AssetServer>, mut windows: Query<&mut Window>) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.cursor.visible = false;

    cmds.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(CURSOR_SIZE),
                height: Val::Px(CURSOR_SIZE),
                ..Default::default()
            },
            image: UiImage {
                texture: assets.load("Cursor.png"),
                ..Default::default()
            },
            ..Default::default()
        },
        Cursor,
    ));
}

pub fn update_cursor(mut cursors: Query<&mut Style, With<Cursor>>, windows: Query<&Window>) {
    let Ok(mut cursor) = cursors.get_single_mut() else {
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(pos) = window.cursor_position() else {
        return;
    };

    cursor.left = Val::Px(pos.x - CURSOR_SIZE / 2.);
    cursor.top = Val::Px(pos.y - CURSOR_SIZE / 2.);
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_camera, setup_cursor))
        .add_systems(
            FixedPostUpdate,
            follow_system::<MainCamera, Player, 10>.after(velocity::apply_velocity),
        )
        .add_systems(Update, zoom_camera)
        .add_systems(Update, update_cursor);
}
