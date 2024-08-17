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
mod elements;

use bevy::{
    asset::AssetMetaCheck,
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

fn create_background(mut cmds: Commands, assets: ResMut<AssetServer>) {
    let texture = assets.load_with_settings("Grid.png", |s| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            }),
            ..Default::default()
        }
    });

    let size = 2048.;

    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::linear_rgb(0.1, 0.1, 0.1),
            rect: Some(Rect::new(0., 0., size, size)),
            ..Default::default()
        },
        texture,
        transform: Transform::from_xyz(0., 0., -1.),
        ..Default::default()
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
        .add_systems(Startup, create_background)
        .run();
}
