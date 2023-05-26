use bevy::{
    prelude::{shape::Cylinder, *},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashMap,
};
use bevy_mod_picking::{
    prelude::{Click, OnPointer, RaycastPickTarget},
    PickableBundle,
};
use hexx::{shapes, ColumnMeshBuilder, Hex, HexLayout};

use super::{events::GridCellClicked, Grid, GridCell, Map, MapMaterials, MapSettings};

pub fn setup_grid(
    mut commands: Commands,
    settings: Res<MapSettings>,
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
    // materials
    let default_material = materials.add(Color::WHITE.into());
    let highlighted_material = materials.add(Color::YELLOW.into());
    // mesh
    let mesh = circle_column(&layout, &settings);
    let mesh_handle = meshes.add(mesh);

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
                    OnPointer::<Click>::send_event::<GridCellClicked>(),
                    // Name::from(format!("{:?}", hex)),
                    GridCell(hex),
                ))
                .id();
            (hex, id)
        })
        .collect();

    commands
        .entity(parent)
        .push_children(&entities.values().copied().collect::<Vec<_>>());

    commands.insert_resource(Map {
        cells: entities,
        cats: default(),
    });
    commands.insert_resource(MapMaterials {
        highlighted_material,
        default_material,
    });
}

fn hexagonal_column(hex_layout: &HexLayout, settings: &MapSettings) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, settings.column_height)
        .without_bottom_face()
        .build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

fn circle_column(hex_layout: &HexLayout, settings: &MapSettings) -> Mesh {
    Cylinder {
        radius: hex_layout.hex_size.x,
        height: settings.column_height,
        ..default()
    }
    .into()
}
