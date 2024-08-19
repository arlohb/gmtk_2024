use std::time::Duration;

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
    pub fn all() -> [ElementInfo; 3] {
        [
            ElementInfo::Hydrogen,
            ElementInfo::Iron,
            ElementInfo::Uranium,
        ]
    }

    pub fn image_path(&self) -> &'static str {
        match self {
            ElementInfo::Hydrogen => "ElementH.png",
            ElementInfo::Iron => "ElementFe.png",
            ElementInfo::Uranium => "ElementU.png",
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            ElementInfo::Hydrogen => 100.,
            ElementInfo::Iron => 300.,
            ElementInfo::Uranium => 100.,
        }
    }

    pub fn firing_time(&self) -> f32 {
        match self {
            ElementInfo::Hydrogen => 1.,
            ElementInfo::Iron => 1.,
            ElementInfo::Uranium => 0.2,
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

        let shooter = Shooter::new(Duration::from_secs_f32(self.firing_time()));

        match self {
            ElementInfo::Hydrogen => parent
                .spawn((
                    sprite_bundle,
                    Atom,
                    Health::new(self.max_health()),
                    Hydrogen,
                    shooter,
                ))
                .with_children(|parent| {
                    parent.spawn(health);
                }),
            ElementInfo::Iron => parent
                .spawn((
                    sprite_bundle,
                    Atom,
                    Health::new(self.max_health()),
                    Iron,
                    shooter,
                ))
                .with_children(|parent| {
                    parent.spawn(health);
                }),
            ElementInfo::Uranium => parent
                .spawn((
                    sprite_bundle,
                    Atom,
                    Health::new(self.max_health()),
                    Uranium,
                    shooter,
                ))
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
