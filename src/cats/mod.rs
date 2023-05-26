use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use tracing::instrument;

use crate::{
    gameplay::events::NewCat,
    grid::{Grid, GridCell, Map, MapSettings},
};

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub enum Cat {
    #[default]
    Kitten,
    Adult,
}

/// Cat figurine
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Meowple;

impl Cat {
    pub fn can_boop(&self, other: Cat) -> bool {
        matches!((self, other), (Cat::Adult, _) | (Cat::Kitten, Cat::Kitten))
    }
}

pub struct CatPlugin;

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Cat>();

        app.add_startup_system(load_gltf);
        app.add_system(spawn_cats.run_if(on_event::<NewCat>()));
    }
}

/// Helper resource for tracking our asset
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct CatAssets {
    mesh: Handle<Gltf>,
    kitten_material: Handle<StandardMaterial>,
    adult_material: Handle<StandardMaterial>,
}

fn load_gltf(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let gltf = assets.load("models/cats.glb");
    commands.insert_resource(CatAssets {
        mesh: gltf,
        kitten_material: materials.add(StandardMaterial {
            base_color: Color::LIME_GREEN,
            ..default()
        }),
        adult_material: materials.add(StandardMaterial {
            base_color: Color::ORANGE,
            ..default()
        }),
    });
}

#[instrument(level = "debug", skip_all)]
fn spawn_cats(
    mut new_cats: EventReader<NewCat>,
    mut commands: Commands,
    settings: Res<MapSettings>,
    map: Res<Map>,
    cells: Query<(&GridCell, &Transform)>,
    cat_assets: Res<CatAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    // GLTF has not loaded? very bad
    let Some(gltf) = assets_gltf.get(&cat_assets.mesh) else {
        panic!("cat meshes not done loading!");
    };

    for NewCat { cat, position, .. } in new_cats.iter() {
        let (mesh, material) = match cat {
            Cat::Kitten => (
                assets_gltfmesh.get(&gltf.meshes[0]).unwrap().primitives[0]
                    .mesh
                    .clone(),
                cat_assets.kitten_material.clone(),
            ),
            Cat::Adult => (
                assets_gltfmesh.get(&gltf.meshes[1]).unwrap().primitives[0]
                    .mesh
                    .clone(),
                cat_assets.adult_material.clone(),
            ),
        };

        let Some(cell) = map.cell_by_hex(*position) else {
            error!(?position, "Cannot spawn cat at position that is not in map");
            continue;
        };
        let (cell, cell_position) = match cells.get(cell) {
            Ok(x) => x,
            Err(error) => {
                error!(
                    ?error,
                    ?position,
                    "Cannot get cell for cat to be spawned on"
                );
                continue;
            }
        };

        let mut transform = *cell_position;
        // cats should sit on top of the cell
        transform.translation.y += settings.column_height;
        // make cats bigger!
        transform.scale = Vec3::splat(2.);

        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            Name::from("Kitten"),
            *cell,
            Meowple,
            *cat,
        ));
    }
}
