use bevy::prelude::*;
use rand::Rng;

pub fn random_in_donut(min_dst: f32, max_dst: f32) -> Vec2 {
    let mut rng = rand::thread_rng();

    let dst = rng.gen_range(min_dst..max_dst);

    Rot2::from_rng(&mut rng) * Vec2::new(dst, 0.)
}
