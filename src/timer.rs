use bevy::{prelude::*, time::Stopwatch};

use crate::{state::GameState, Player};

#[derive(Resource, Default)]
pub struct GameTimer(pub Stopwatch);

pub fn game_timer_system(mut timer: ResMut<GameTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

pub fn game_end_system(
    timer: Res<GameTimer>,
    players: Query<(), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if players.iter().count() == 0 {
        let duration = timer.0.elapsed_secs();

        let mins = (duration / 60.).floor() as u32;
        let secs = duration % 60.;

        info!("You survived {} mins and {:.2} secs", mins, secs);

        next_state.set(GameState::Death);
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>().add_systems(
        Update,
        (game_timer_system, game_end_system)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}
