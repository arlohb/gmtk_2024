use bevy::prelude::*;

#[derive(Component)]
pub struct HealthSprite;

#[derive(Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            health,
            max_health: health,
        }
    }
}

pub fn health_texture_system(
    parents: Query<&Health, With<Children>>,
    mut children: Query<(&mut Sprite, &Parent), With<HealthSprite>>,
) {
    for (mut sprite, parent) in &mut children {
        let Ok(health) = parents.get(parent.get()) else {
            continue;
        };

        let percent = health.health / health.max_health;
        let texture_index = (percent * 8.).ceil() - 1.;

        sprite.rect = Some(Rect::new(
            texture_index * 32.,
            0.,
            (texture_index + 1.) * 32.,
            32.,
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, health_texture_system);
}
