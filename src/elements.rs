use bevy::prelude::*;

use crate::{
    health::{Health, HealthSprite},
    shooting::Shooter,
};

#[derive(Component, Clone)]
pub struct Atom;

#[derive(Clone, Copy)]
pub enum ElementInfo {
    Hydrogen,
    Iron,
    Uranium,
}

impl ElementInfo {
    pub fn image_path(&self) -> &'static str {
        match self {
            ElementInfo::Hydrogen => "ElementH.png",
            ElementInfo::Iron => "ElementFe.png",
            ElementInfo::Uranium => "ElementU.png",
        }
    }

    pub fn build(
        &self,
        parent: &mut ChildBuilder,
        assets: &AssetServer,
        offset: Vec2,
        is_player: bool,
    ) {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::linear_rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(offset.x, offset.y, 0.),
            texture: assets.load(self.image_path()),
            ..Default::default()
        };

        let health = (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::linear_rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(128., 128.)),
                    rect: Some(Rect::new(0., 0., 32., 32.)),
                    ..Default::default()
                },
                texture: assets.load(if is_player {
                    "Health.png"
                } else {
                    "HealthEnemy.png"
                }),
                ..Default::default()
            },
            HealthSprite,
        );

        match self {
            ElementInfo::Hydrogen => parent
                .spawn((sprite_bundle, Atom, Health::new(100.), Hydrogen))
                .with_children(|parent| {
                    parent.spawn(health);
                }),
            ElementInfo::Iron => parent
                .spawn((sprite_bundle, Atom, Health::new(0.), Iron))
                .with_children(|parent| {
                    parent.spawn(health);
                }),
            ElementInfo::Uranium => parent
                .spawn((sprite_bundle, Atom, Health::new(100.), Uranium, Shooter))
                .with_children(|parent| {
                    parent.spawn(health);
                }),
        };
    }
}

#[derive(Component)]
pub struct Hydrogen;

#[derive(Component)]
pub struct Iron;

#[derive(Component)]
pub struct Uranium;
