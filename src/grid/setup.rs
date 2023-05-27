use bevy::{
    prelude::{shape::Cylinder, *},
    utils::HashMap,
};
use bevy_mod_picking::{
    prelude::{Click, OnPointer, Out, Over, RaycastPickTarget},
    PickableBundle,
};
use hexx::{shapes, Hex, HexLayout};

use crate::events::GridCellClicked;

use super::{Grid, GridCell, Hovered, Map, MapSettings, Platform};

#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CellMaterials {
    pub default: Handle<StandardMaterial>,
    pub hovered_by_player: [Handle<StandardMaterial>; 2],
}

pub fn setup_grid(
    mut commands: Commands,
    settings: Res<MapSettings>,
    mut map: ResMut<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let MapSettings {
        cell_size: hex_size,
        map_radius,
        ..
    } = *settings;

    let layout = HexLayout {
        hex_size,
        ..default()
    };
    let default_material = materials.add(Color::WHITE.into());
    let mesh = circle_column(&layout, &settings);
    let mesh_handle = meshes.add(mesh);

    commands.insert_resource(CellMaterials {
        default: default_material.clone(),
        hovered_by_player: [
            materials.add(Color::LIME_GREEN.into()),
            materials.add(Color::ORANGE.into()),
        ],
    });

    let parent = commands
        .spawn((SpatialBundle { ..default() }, Name::from("Grid"), Grid))
        .id();

    let entities: HashMap<_, _> = shapes::hexagon(Hex::ZERO, map_radius)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(pos.x, 0.0, pos.y)
                            .with_scale(Vec3::splat(0.7)),
                        mesh: mesh_handle.clone(),
                        material: default_material.clone(),
                        ..default()
                    },
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    OnPointer::<Over>::target_insert(Hovered),
                    OnPointer::<Out>::target_remove::<Hovered>(),
                    OnPointer::<Click>::send_event::<GridCellClicked>(),
                    // Name::from(format!("{:?}", hex)),
                    GridCell(hex),
                    Platform,
                ))
                .id();
            (hex, id)
        })
        .collect();

    commands
        .entity(parent)
        .push_children(&entities.values().copied().collect::<Vec<_>>());

    map.cells = entities;
}

fn circle_column(hex_layout: &HexLayout, settings: &MapSettings) -> Mesh {
    Cylinder {
        radius: hex_layout.hex_size.x,
        height: settings.column_height,
        ..default()
    }
    .into()
}
