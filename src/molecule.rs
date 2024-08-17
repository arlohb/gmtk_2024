use bevy::prelude::*;

use crate::elements::ElementInfo;

#[derive(Component)]
pub struct Molecule {
    pub elements: Vec<ElementInfo>,
}
