use bevy::{prelude::*, time::Stopwatch};

use crate::{state::GameState, Player};

#[derive(Resource, Default)]
pub struct GameTimer(pub Stopwatch);

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

pub fn death_enter_system(mut cmds: Commands, timer: Res<GameTimer>) {
    let duration = timer.0.elapsed_secs();

    let mins = (duration / 60.).floor() as u32;
    let secs = duration % 60.;

    cmds.spawn(TextBundle {
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
}

pub fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>().add_systems(
        Update,
        (game_timer_system, game_end_system)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}
