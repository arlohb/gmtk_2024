use bevy::prelude::*;

use crate::{
    elements::{Atom, ElementInfo},
    enemy::Enemy,
    energy::Energy,
    health::Health,
    Player,
};

#[derive(Event)]
pub enum BuildMolecule {
    Create {
        target: Entity,
    },
    Add {
        target: Entity,
        element: ElementInfo,
    },
    RemoveAtom {
        target: Entity,
        atom: Entity,
    },
}

#[derive(Component)]
pub struct Molecule {
    pub elements: Vec<ElementInfo>,
}

impl Molecule {
    pub fn collision_radius(&self) -> f32 {
        (match self.elements.len() {
            1 => 32.,
            2 => 56.,
            4 => 64.,
            n => n as f32 * 24.,
        }) + 24.
    }
}

fn create_polygon(points: usize) -> Vec<Vec2> {
    let start_point = match points {
        1 => Vec2::new(0., 0.),
        2 => Vec2::new(-56., 0.),
        4 => Vec2::new(64., 64.),
        _ => Vec2::new(0., points as f32 * 24.),
    };

    (0..points)
        .map(|i| Rot2::degrees(i as f32 * -360. / points as f32) * start_point)
        .collect()
}

pub fn build_molecules_system(
    mut events: EventReader<BuildMolecule>,
    assets: Res<AssetServer>,
    mut cmds: Commands,
    mut molecules: Query<(Entity, &mut Molecule, Option<&Player>, Option<&Children>)>,
    mut child_transforms: Query<&mut Transform, With<Parent>>,
) {
    for event in events.read() {
        match *event {
            BuildMolecule::Create { target } => {
                cmds.entity(target).with_children(|parent| {
                    let Ok((_, molecule, player, _)) = molecules.get(target) else {
                        return;
                    };

                    let offsets = create_polygon(molecule.elements.len());

                    molecule
                        .elements
                        .iter()
                        .enumerate()
                        .for_each(|(i, element)| {
                            element.build(parent, &assets, offsets[i], player.is_some());
                        });
                });
            }
            BuildMolecule::Add { target, element } => {
                let Ok((_, mut molecule, player, Some(old_children))) = molecules.get_mut(target)
                else {
                    return;
                };

                molecule.elements.push(element);

                let offsets = create_polygon(molecule.elements.len());

                for (i, child) in old_children.iter().enumerate() {
                    let pos = &mut child_transforms.get_mut(*child).unwrap().translation;
                    let offset = offsets[i];
                    pos.x = offset.x;
                    pos.y = offset.y;
                }

                cmds.entity(target).with_children(|parent| {
                    element.build(parent, &assets, *offsets.last().unwrap(), player.is_some());
                });
            }
            BuildMolecule::RemoveAtom { target, atom } => {
                let Ok((entity, mut molecule, _, Some(old_children))) = molecules.get_mut(target)
                else {
                    return;
                };

                if let Some((mut index, _)) = old_children
                    .iter()
                    .enumerate()
                    .find(|(_, child)| **child == atom)
                {
                    // If this is the only child
                    if molecule.elements.len() == 1 {
                        cmds.entity(entity).despawn_recursive();
                        continue;
                    }

                    // If removal is invalid
                    if index >= molecule.elements.len() {
                        index = molecule.elements.len() - 1;
                    }

                    molecule.elements.remove(index);
                    let mut atom_cmds = cmds.entity(atom);
                    atom_cmds.remove_parent();
                    atom_cmds.despawn_recursive();

                    let offsets = create_polygon(molecule.elements.len());

                    for (i, child) in old_children.iter().enumerate().filter(|(i, _)| *i != index) {
                        let pos = &mut child_transforms.get_mut(*child).unwrap().translation;
                        let offset = offsets
                            .get(if i > index { i - 1 } else { i })
                            .unwrap_or_else(|| &offsets[offsets.len() - 1]);
                        pos.x = offset.x;
                        pos.y = offset.y;
                    }
                }
            }
        }
    }
}

pub fn molecule_health_system(
    enemies: Query<&Enemy>,
    query: Query<(Entity, &Health, &Parent), With<Atom>>,
    mut build_molecule_event: EventWriter<BuildMolecule>,
    mut energy: ResMut<Energy>,
) {
    for (entity, health, parent) in query.iter() {
        if health.health <= 0. {
            build_molecule_event.send(BuildMolecule::RemoveAtom {
                target: parent.get(),
                atom: entity,
            });

            if enemies.contains(parent.get()) {
                energy.0 += 10.;
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<BuildMolecule>().add_systems(
        Update,
        (molecule_health_system, build_molecules_system).chain(),
    );
}
