use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct TimeToLive(Timer);

impl TimeToLive {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut ttl) in &mut query {
        if ttl.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, time_to_live);
}
