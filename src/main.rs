mod movement;
pub use movement::*;
mod velocity;
pub use velocity::*;
mod camera;
pub use camera::*;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

fn create_player(mut cmds: Commands, assets: ResMut<AssetServer>) {
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
        ..Default::default()
    });
}

fn main() {
    App::new()
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
        .add_systems(Startup, (create_player, setup_camera, create_background))
        .add_systems(FixedUpdate, movement_system)
        .add_systems(
            FixedPostUpdate,
            (apply_velocity, follow_player.after(apply_velocity)),
        )
        .run();
}
