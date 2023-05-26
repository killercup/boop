use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};

pub struct CatAssetPlugin;

impl Plugin for CatAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_gltf);
        app.add_system(spawn_cats);
    }
}

/// Helper resource for tracking our asset
#[derive(Resource)]
struct CatAssets(Handle<Gltf>);

fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("models/cats.glb");
    commands.insert_resource(CatAssets(gltf));
}

fn spawn_cats(
    mut commands: Commands,
    my: Res<CatAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rendered: Local<bool>,
) {
    if *rendered {
        return;
    }
    // if the GLTF has loaded, we can navigate its contents
    let Some(gltf) = assets_gltf.get(&my.0) else {
        debug!("loading");
        return;
    };

    debug!(?gltf, "gltf");
    *rendered = true;

    commands.spawn(PbrBundle {
        mesh: assets_gltfmesh.get(&gltf.meshes[0]).unwrap().primitives[0]
            .mesh
            .clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::LIME_GREEN,
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 0.0, 0.0).with_scale(Vec3::splat(2.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: assets_gltfmesh.get(&gltf.meshes[1]).unwrap().primitives[0]
            .mesh
            .clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::ORANGE,
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 0.0, 0.0).with_scale(Vec3::splat(2.)),
        ..default()
    });
}
