mod movement;
pub use movement::*;
mod velocity;
pub use velocity::Velocity;
mod camera;
pub use camera::MainCamera;
mod time_to_live;
pub use time_to_live::TimeToLive;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

fn create_player(mut cmds: Commands, assets: Res<AssetServer>) {
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
    ));
}

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
        .add_systems(Startup, (create_player, create_background))
        .run();
}
