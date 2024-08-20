use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct MenuCleanup;

#[derive(Component)]
pub struct PlayButton;

pub fn setup_menu(mut cmds: Commands) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                top: Val::Vh(30.),
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        },
        MenuCleanup,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            style: Style {
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection::new(
                    "Atomic Warfare",
                    TextStyle {
                        font_size: 64.,
                        ..Default::default()
                    },
                )],
                justify: JustifyText::Center,
                ..Default::default()
            },
            ..Default::default()
        });

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(16.)),
                        margin: UiRect::top(Val::Vh(20.)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                    ..Default::default()
                },
                PlayButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Play",
                            TextStyle {
                                font_size: 64.,
                                ..Default::default()
                            },
                        )],
                        justify: JustifyText::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    });
}

pub fn play_system(
    interactions: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if let Interaction::Pressed = interaction {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn cleanup_menu(mut cmds: Commands, entities: Query<Entity, With<MenuCleanup>>) {
    let mut despawn = |entity| cmds.entity(entity).despawn_recursive();
    entities.iter().for_each(&mut despawn);
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(Update, play_system.run_if(in_state(GameState::Menu)));
}
