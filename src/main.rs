#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod movement;
pub use movement::*;
mod velocity;
pub use velocity::Velocity;
mod camera;
pub use camera::MainCamera;
mod time_to_live;
pub use time_to_live::TimeToLive;
mod player;
mod shooting;
mod shop;
pub use player::Player;
mod collision;
mod death;
mod elements;
mod enemy;
mod energy;
mod follow;
mod health;
mod molecule;
mod powerup;
mod state;
mod timer;
mod utils;
mod wave;

use bevy::{
    asset::AssetMetaCheck,
    audio::PlaybackMode,
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

fn create_background(mut cmds: Commands, assets: ResMut<AssetServer>) {
    let texture = assets.load_with_settings("Background.png", |s| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            }),
            ..Default::default()
        }
    });

    let size = 2f32.powi(16);

    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::linear_rgb(1., 1., 1.),
            rect: Some(Rect::new(0., 0., size, size)),
            ..Default::default()
        },
        texture,
        transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::new(4., 4., 1.)),
        ..Default::default()
    });
}

fn setup_music(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn(AudioBundle {
        source: assets.load("music.mp3"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    });
}

fn main() {
    App::new()
        // Enable ambiguity detection
        // Have to ignore the warning for time_system and event_update_system
        .configure_schedules(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..Default::default()
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Trunk doesn't return 404 if a file isn't there,
                    // so the meta check will assume it should be there,
                    // which it isn't
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .add_plugins(camera::plugin)
        .add_plugins(time_to_live::plugin)
        .add_plugins(velocity::plugin)
        .add_plugins(movement::plugin)
        .add_plugins(shooting::plugin)
        .add_plugins(shop::plugin)
        .add_plugins(player::plugin)
        .add_plugins(molecule::plugin)
        .add_plugins(enemy::plugin)
        .add_plugins(health::plugin)
        .add_plugins(wave::plugin)
        .add_plugins(energy::plugin)
        .add_plugins(powerup::plugin)
        .add_plugins(timer::plugin)
        .add_plugins(state::plugin)
        .add_plugins(death::plugin)
        .add_systems(Startup, (create_background, setup_music))
        .run();
}
