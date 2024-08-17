use bevy::prelude::*;

pub enum ElementInfo {
    Uranium,
}

impl ElementInfo {
    pub fn image_path(&self) -> &'static str {
        match self {
            ElementInfo::Uranium => "ElementU.png",
        }
    }

    pub fn build(&self, assets: &AssetServer) -> impl Bundle {
        match self {
            ElementInfo::Uranium => (
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::linear_rgb(1., 1., 1.),
                        custom_size: Some(Vec2::new(64., 64.)),
                        ..Default::default()
                    },
                    texture: assets.load(self.image_path()),
                    ..Default::default()
                },
                Uranium,
            ),
        }
    }
}

#[derive(Component)]
pub struct Uranium;

pub fn build_elements(elements: &[ElementInfo], assets: &AssetServer) -> Vec<impl Bundle> {
    elements
        .iter()
        .map(|element| element.build(assets))
        .collect()
}
