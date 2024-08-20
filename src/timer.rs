use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    state::{GameState, PlayingCleanup},
    Player,
};

#[derive(Resource, Default)]
pub struct GameTimer(pub Stopwatch);

#[derive(Resource, Default)]
pub struct HighScore(pub Duration);

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
pub struct TimerText;

pub fn setup_timer(mut cmds: Commands) {
    cmds.spawn((
        TextBundle {
            style: Style {
                top: Val::Percent(10.),
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection::new(
                        "".to_string(),
                        TextStyle {
                            font_size: 64.,
                            ..Default::default()
                        },
                    ),
                    TextSection::new(
                        "".to_string(),
                        TextStyle {
                            font_size: 48.,
                            ..Default::default()
                        },
                    ),
                ],
                justify: JustifyText::Center,
                ..Default::default()
            },
            ..Default::default()
        },
        TimerText,
        PlayingCleanup,
    ));
}

pub fn update_timer(
    mut texts: Query<&mut Text, With<TimerText>>,
    timer: Res<GameTimer>,
    highscore: Res<HighScore>,
) {
    let Ok(mut text) = texts.get_single_mut() else {
        return;
    };

    let duration = timer.0.elapsed_secs();

    let mins = (duration / 60.).floor() as u32;
    let secs = duration % 60.;

    text.sections[0].value = format!("{} mins {:.2} secs\n\n", mins, secs);

    let duration = highscore.0.as_secs_f32();

    let mins = (duration / 60.).floor() as u32;
    let secs = duration % 60.;
    text.sections[1].value = format!("Highscore: {} mins {:.2} secs", mins, secs);
}

pub fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>()
        .init_resource::<HighScore>()
        .add_systems(
            Update,
            (game_timer_system, update_timer, game_end_system)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (reset_timer_system, setup_timer),
        );
}
