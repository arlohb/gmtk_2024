use bevy::prelude::*;

use crate::{
    camera::{setup_cursor, Cursor},
    death::death_enter_system,
    enemy::Enemy,
    energy::reset_energy,
    powerup::Powerup,
    shooting::Bullet,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Death,
}

#[derive(Component)]
pub struct PlayingCleanup;

pub fn cleanup_system(
    mut cmds: Commands,
    entities: Query<
        Entity,
        Or<(
            With<Enemy>,
            With<Bullet>,
            With<Cursor>,
            With<Powerup>,
            With<PlayingCleanup>,
        )>,
    >,
    mut windows: Query<&mut Window>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.cursor.visible = true;

    let mut despawn = |entity| cmds.entity(entity).despawn_recursive();
    entities.iter().for_each(&mut despawn);
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), (setup_cursor, reset_energy))
        .add_systems(OnExit(GameState::Playing), cleanup_system)
        .add_systems(OnEnter(GameState::Death), death_enter_system);
}
