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

pub fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>()
        .add_systems(
            Update,
            (game_timer_system, game_end_system)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Playing), reset_timer_system);
}
