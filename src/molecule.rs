use bevy::prelude::*;

use crate::{
    elements::{Atom, ElementInfo},
    health::Health,
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

fn create_polygon(points: usize) -> Vec<Vec2> {
    let start_point = match points {
        1 => Vec2::new(0., 0.),
        2 => Vec2::new(-36., 0.),
        4 => Vec2::new(48., 48.),
        _ => Vec2::new(0., points as f32 * 16.),
    };

    (0..points)
        .map(|i| Rot2::degrees(i as f32 * -360. / points as f32) * start_point)
        .collect()
}

pub fn build_molecules_system(
    mut events: EventReader<BuildMolecule>,
    assets: Res<AssetServer>,
    mut cmds: Commands,
    mut molecules: Query<(&mut Molecule, Option<&Children>)>,
    mut child_transforms: Query<&mut Transform, With<Parent>>,
) {
    for event in events.read() {
        match *event {
            BuildMolecule::Create { target } => {
                cmds.entity(target).with_children(|parent| {
                    let Ok((molecule, _)) = molecules.get(target) else {
                        return;
                    };

                    let offsets = create_polygon(molecule.elements.len());

                    molecule
                        .elements
                        .iter()
                        .enumerate()
                        .for_each(|(i, element)| {
                            element.build(parent, &assets, offsets[i]);
                        });
                });
            }
            BuildMolecule::Add { target, element } => {
                let Ok((mut molecule, Some(old_children))) = molecules.get_mut(target) else {
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
                    element.build(parent, &assets, *offsets.last().unwrap());
                });
            }
            BuildMolecule::RemoveAtom { target, atom } => {
                let Ok((mut molecule, Some(old_children))) = molecules.get_mut(target) else {
                    return;
                };

                if let Some((index, _)) = old_children
                    .iter()
                    .enumerate()
                    .find(|(_, child)| **child == atom)
                {
                    molecule.elements.remove(index);
                    cmds.entity(atom).remove_parent().despawn();

                    let offsets = create_polygon(molecule.elements.len());

                    for (i, child) in old_children.iter().enumerate().filter(|(i, _)| *i != index) {
                        let pos = &mut child_transforms.get_mut(*child).unwrap().translation;
                        let offset = offsets[if i > index { i - 1 } else { i }];
                        pos.x = offset.x;
                        pos.y = offset.y;
                    }
                }
            }
        }
    }
}

pub fn molecule_health_system(
    query: Query<(Entity, &Health, &Parent), With<Atom>>,
    mut build_molecule_event: EventWriter<BuildMolecule>,
) {
    for (entity, health, parent) in query.iter() {
        if health.health <= 0. {
            build_molecule_event.send(BuildMolecule::RemoveAtom {
                target: parent.get(),
                atom: entity,
            });
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<BuildMolecule>().add_systems(
        Update,
        (molecule_health_system, build_molecules_system).chain(),
    );
}
