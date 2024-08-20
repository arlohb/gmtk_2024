use bevy::prelude::*;

use crate::{
    state::GameState,
    timer::{GameTimer, HighScore},
};

#[derive(Component)]
pub struct RespawnButton;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct DeathCleanup;

pub fn death_enter_system(
    mut cmds: Commands,
    timer: Res<GameTimer>,
    mut highscore: ResMut<HighScore>,
) {
    let duration = timer.0.elapsed_secs();

    let mins = (duration / 60.).floor() as u32;
    let secs = duration % 60.;

    let new_highscore = if timer.0.elapsed() > highscore.0 {
        highscore.0 = timer.0.elapsed();
        true
    } else {
        false
    };

    let h_duration = highscore.0.as_secs_f32();
    let h_mins = (h_duration / 60.).floor() as u32;
    let h_secs = h_duration % 60.;

    cmds.spawn((
        NodeBundle {
            style: Style {
                top: Val::Percent(20.),
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        },
        DeathCleanup,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            style: Style {
                top: Val::Percent(40.),
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection::new(
                    format!(
                        "{}You survived\n{} mins {:.2} secs",
                        if new_highscore {
                            "New Highscore!\n".to_string()
                        } else {
                            format!("Highscore: {} mins {:.2} secs\n\n", h_mins, h_secs)
                        },
                        mins,
                        secs
                    ),
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
                RespawnButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Respawn?",
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

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(16.)),
                        margin: UiRect::top(Val::Px(24.)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                    ..Default::default()
                },
                MenuButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Menu",
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

pub fn respawn_button_system(
    interactions: Query<&Interaction, (With<RespawnButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if let Interaction::Pressed = interaction {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn menu_button_system(
    interactions: Query<&Interaction, (With<MenuButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if let Interaction::Pressed = interaction {
            next_state.set(GameState::Menu);
        }
    }
}

pub fn death_cleanup_system(mut cmds: Commands, entities: Query<Entity, With<DeathCleanup>>) {
    let mut despawn = |entity| cmds.entity(entity).despawn_recursive();
    entities.iter().for_each(&mut despawn);
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        respawn_button_system.run_if(in_state(GameState::Death)),
    )
    .add_systems(
        Update,
        menu_button_system.run_if(in_state(GameState::Death)),
    )
    .add_systems(OnExit(GameState::Death), death_cleanup_system);
}
