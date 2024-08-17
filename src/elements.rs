use bevy::prelude::*;

use crate::health::Health;

#[derive(Component)]
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

    pub fn build(&self, parent: &mut ChildBuilder, assets: &AssetServer, offset: Vec2) {
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
        match self {
            ElementInfo::Hydrogen => {
                parent.spawn((sprite_bundle, Atom, Health::new(100.), Hydrogen))
            }
            ElementInfo::Iron => parent.spawn((sprite_bundle, Atom, Health::new(0.), Iron)),
            ElementInfo::Uranium => parent.spawn((sprite_bundle, Atom, Health::new(100.), Uranium)),
        };
    }
}

#[derive(Component)]
pub struct Hydrogen;

#[derive(Component)]
pub struct Iron;

#[derive(Component)]
pub struct Uranium;
