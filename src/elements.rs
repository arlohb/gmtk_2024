use bevy::{ecs::system::SystemId, prelude::*};

use crate::Player;

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
            ElementInfo::Hydrogen => parent.spawn((sprite_bundle, Hydrogen)),
            ElementInfo::Iron => parent.spawn((sprite_bundle, Iron)),
            ElementInfo::Uranium => parent.spawn((sprite_bundle, Uranium)),
        };
    }
}

#[derive(Component)]
pub struct Hydrogen;

#[derive(Component)]
pub struct Iron;

#[derive(Component)]
pub struct Uranium;

pub fn build_elements(
    assets: Res<AssetServer>,
    players: Query<(Entity, &Player, Option<&Children>), With<Player>>,
    mut cmds: Commands,
) {
    let (player_id, player, old_children) = players.single();

    cmds.entity(player_id)
        .clear_children()
        .with_children(|parent| {
            player.elements.iter().enumerate().for_each(|(i, element)| {
                element.build(parent, &assets, Vec2::new(72., 0.) * i as f32)
            });
        });

    if let Some(old_children) = old_children {
        old_children.iter().for_each(|child| {
            // return;
            cmds.entity(*child).despawn();
        });
    }
}

#[derive(Resource)]
pub struct BuildElements(pub SystemId);

impl FromWorld for BuildElements {
    fn from_world(world: &mut World) -> Self {
        Self(world.register_system(build_elements))
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<BuildElements>();
}
