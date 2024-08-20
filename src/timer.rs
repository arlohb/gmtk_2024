use bevy::{prelude::*, time::Stopwatch};

use crate::{state::GameState, Player};

#[derive(Resource, Default)]
pub struct GameTimer(pub Stopwatch);

pub fn reset_timer_system(mut timer: ResMut<GameTimer>) {
    timer.0.reset();
}

pub fn game_timer_system(mut timer: ResMut<GameTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

pub fn game_end_system(
    players: Query<(), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if players.iter().count() == 0 {
        next_state.set(GameState::Death);
    }
}

#[derive(Component)]
pub struct RespawnButton;

#[derive(Component)]
pub struct DeathCleanup;

pub fn death_enter_system(mut cmds: Commands, timer: Res<GameTimer>) {
    let duration = timer.0.elapsed_secs();

    let mins = (duration / 60.).floor() as u32;
    let secs = duration % 60.;

    cmds.spawn((
        NodeBundle {
            style: Style {
                top: Val::Percent(40.),
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
                    format!("You survived\n{} mins and {:.2} secs", mins, secs),
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
                        margin: UiRect::top(Val::Px(64.)),
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
    });
}

pub fn respawn_system(
    interactions: Query<&Interaction, (With<RespawnButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if let Interaction::Pressed = interaction {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn death_cleanup_system(mut cmds: Commands, entities: Query<Entity, With<DeathCleanup>>) {
    let mut despawn = |entity| cmds.entity(entity).despawn_recursive();
    entities.iter().for_each(&mut despawn);
}

pub fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>()
        .add_systems(
            Update,
            (game_timer_system, game_end_system)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, respawn_system.run_if(in_state(GameState::Death)))
        .add_systems(OnEnter(GameState::Playing), reset_timer_system)
        .add_systems(OnExit(GameState::Death), death_cleanup_system);
}
