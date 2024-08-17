use bevy::prelude::*;

#[derive(Component)]
pub struct ShopButton(String);

pub fn shop_button_system(query: Query<(&Interaction, &ShopButton), Changed<Interaction>>) {
    for (interaction, ShopButton(element)) in &query {
        if let Interaction::Pressed = interaction {
            info!("{}", element);
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
        for image in ["ElementH.png", "ElementFe.png", "ElementU.png"] {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(64.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ShopButton(image.to_string()),
                ))
                .with_children(|button| {
                    button.spawn(ImageBundle {
                        image: UiImage {
                            texture: assets.load(image),
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
