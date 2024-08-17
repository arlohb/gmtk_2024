use bevy::prelude::*;

use crate::elements::ElementInfo;

#[derive(Event)]
pub struct BuildMolecule(pub Entity);

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
    molecules: Query<(&Molecule, Option<&Children>)>,
) {
    for &BuildMolecule(entity) in events.read() {
        let Ok((molecule, old_children)) = molecules.get(entity) else {
            return;
        };

        cmds.entity(entity)
            .clear_children()
            .with_children(|parent| {
                let offsets = create_polygon(molecule.elements.len());

                molecule
                    .elements
                    .iter()
                    .enumerate()
                    .for_each(|(i, element)| {
                        element.build(parent, &assets, offsets[i]);
                    });
            });

        if let Some(old_children) = old_children {
            old_children.iter().for_each(|child| {
                cmds.entity(*child).despawn();
            });
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<BuildMolecule>()
        .add_systems(Update, build_molecules_system);
}
