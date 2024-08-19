use bevy::prelude::*;
use rand::Rng;

use crate::{
    collision::{collision_system, CollisionEvent},
    elements::ElementInfo,
    energy::Energy,
    molecule::BuildMolecule,
    utils::random_in_donut,
    Player,
};

#[derive(Component, Clone)]
pub enum Powerup {
    AddAtom(ElementInfo),
}

pub fn spawn_powerup_system(
    mut cmds: Commands,
    players: Query<&Transform, With<Player>>,
    mut energy: ResMut<Energy>,
    assets: Res<AssetServer>,
    mut count: Local<u32>,
) {
    let needed_energy = 20. + 1.5f32.powi(*count as i32);

    if energy.0 >= needed_energy {
        energy.0 = 0.;
        *count += 1;

        let Ok(player) = players.get_single() else {
            return;
        };
        let center = player.translation.xy();

        let element = {
            let mut rng = rand::thread_rng();
            let all = ElementInfo::all();
            let index = rng.gen_range(0..all.len());
            all[index]
        };

        cmds.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::linear_rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(64., 64.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(
                    (center + random_in_donut(600., 2000.)).extend(0.),
                ),
                texture: assets.load(element.image_path()),
                ..Default::default()
            },
            Powerup::AddAtom(element),
        ));
    }
}

#[derive(Component)]
pub struct PowerupArrow;

pub fn setup_powerup_arrow(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        ImageBundle {
            style: Style {
                left: Val::Percent(50.),
                top: Val::Px(0.),
                width: Val::Px(64.),
                height: Val::Px(64.),
                ..Default::default()
            },
            image: UiImage {
                texture: assets.load("Arrow.png"),
                color: Color::linear_rgba(1., 1., 1., 0.1),
                ..Default::default()
            },
            ..Default::default()
        },
        PowerupArrow,
    ));
}

pub fn powerup_arrow_system(
    powerups: Query<(&Transform, &ViewVisibility), (With<Powerup>, Without<PowerupArrow>)>,
    players: Query<&Transform, (With<Player>, Without<PowerupArrow>)>,
    mut arrows: Query<(&mut Transform, &mut Style, &mut UiImage), With<PowerupArrow>>,
    windows: Query<&Window>,
    mut last_delta: Local<Vec2>,
) {
    let Ok((mut arrow_transform, mut arrow_style, mut arrow_image)) = arrows.get_single_mut()
    else {
        return;
    };

    let Ok(player) = players.get_single() else {
        return;
    };
    let player = player.translation.xy();

    let Ok(window) = windows.get_single() else {
        return;
    };

    let image_padding = 32.;
    let padding = 32.;

    let width = window.width() - image_padding * 2. - padding * 2.;
    let height = window.height() - image_padding * 2. - padding * 2.;

    // Get the closest powerup that isn't visible
    let Some((powerup, _)) = powerups
        .iter()
        .filter(|(_, is_visible)| !is_visible.get())
        .min_by(|(a, _), (b, _)| {
            (a.translation.xy() - player)
                .length()
                .total_cmp(&(b.translation.xy() - player).length())
        })
    else {
        arrow_image.color = Color::linear_rgba(1., 1., 1., arrow_image.color.alpha().lerp(0., 0.1));
        return;
    };

    let delta = (powerup.translation.xy() - player).normalize();
    let delta = if arrow_image.color.alpha() <= 0.01 {
        delta
    } else {
        last_delta.lerp(delta, 0.05)
    };
    *last_delta = delta;

    arrow_image.color = Color::linear_rgba(1., 1., 1., arrow_image.color.alpha().lerp(1., 0.04));

    let abs_delta = delta.abs();

    let dx_by_width = 0.5 * width / abs_delta.x;
    let dy_by_height = 0.5 * height / abs_delta.y;

    let mut delta_scaled = delta * dx_by_width.min(dy_by_height);
    delta_scaled.y *= -1.;

    let screen_coords = delta_scaled + Vec2::new(width / 2. + padding, height / 2. + padding);

    arrow_style.left = Val::Px(screen_coords.x);
    arrow_style.top = Val::Px(screen_coords.y);

    let rotation = delta.x.atan2(delta.y);
    arrow_transform.rotation = Quat::from_rotation_z(rotation);
}

pub fn powerup_player_collision_system(
    mut events: EventReader<CollisionEvent<Powerup, Player>>,
    mut build_molecule_event: EventWriter<BuildMolecule>,
    mut cmds: Commands,
) {
    for event in events.read() {
        match event.a_comp {
            Powerup::AddAtom(element) => build_molecule_event.send(BuildMolecule::Add {
                target: event.b_id,
                element,
            }),
        };

        cmds.entity(event.a_id).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent<Powerup, Player>>()
        .add_systems(Startup, setup_powerup_arrow)
        .add_systems(Update, spawn_powerup_system)
        .add_systems(Update, powerup_arrow_system)
        .add_systems(
            Update,
            (
                collision_system::<Powerup, Player>,
                powerup_player_collision_system,
            )
                .chain(),
        );
}
