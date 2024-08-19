use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Energy(pub f32);

pub fn reset_energy(mut energy: ResMut<Energy>) {
    energy.0 = 0.;
}

pub fn plugin(app: &mut App) {
    app.init_resource::<Energy>();
}
