use bevy::prelude::*;

use crate::{
    elements::{BuildElements, ElementInfo},
    molecule::Molecule,
    Player,
};

#[derive(Component)]
pub struct ShopButton(ElementInfo);

pub fn shop_button_system(
    query: Query<(&Interaction, &ShopButton), Changed<Interaction>>,
    mut players: Query<&mut Molecule, With<Player>>,
    build_elements: Res<BuildElements>,
    mut cmds: Commands,
) {
    for (interaction, ShopButton(element)) in &query {
        if let Interaction::Pressed = interaction {
            let mut player = players.single_mut();
            player.elements.push(*element);
            cmds.run_system(build_elements.0);
        }
    }
}

pub fn shop_setup(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn(NodeBundle {
        style: Style {
            left: Val::Percent(70.),
            top: Val::Percent(80.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    })
    .with_children(|parent| {
        for element in [
            ElementInfo::Hydrogen,
            ElementInfo::Iron,
            ElementInfo::Uranium,
        ] {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(64.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ShopButton(element),
                ))
                .with_children(|button| {
                    button.spawn(ImageBundle {
                        image: UiImage {
                            texture: assets.load(element.image_path()),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        }
    });
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, shop_setup)
        .add_systems(Update, shop_button_system);
}
