use bevy::prelude::*;

use crate::{
    camera::{setup_cursor, Cursor},
    enemy::Enemy,
    energy::reset_energy,
    shooting::Bullet,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
    Death,
}

pub fn cleanup_system(
    mut cmds: Commands,
    entities: Query<Entity, Or<(With<Enemy>, With<Bullet>, With<Cursor>)>>,
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
        .add_systems(OnExit(GameState::Playing), cleanup_system);
}
