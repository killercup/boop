use std::time::Duration;

use crate::GameState;
use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        );
        app.add_collection_to_loading_state::<_, FontAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, CatModel>(GameState::Loading);

        app.add_system(spawn_loading_animation.in_schedule(OnEnter(GameState::Loading)));
        app.add_system(remove_loading_animation.in_schedule(OnExit(GameState::Loading)));
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct LoadingLayer;

fn spawn_loading_animation(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let bounce_forever = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::new(0., 5., 0.),
            end: Vec3::new(0., -5., 0.),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    fn hexagonal_column() -> Mesh {
        use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
        use hexx::{ColumnMeshBuilder, HexLayout};

        let mesh_info = ColumnMeshBuilder::new(
            &HexLayout {
                hex_size: Vec2::splat(2.),
                ..default()
            },
            1.,
        )
        .without_bottom_face()
        .build();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
        mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
        mesh
    }

    let material = materials.add(StandardMaterial {
        base_color: Color::ORANGE,
        metallic: 0.3,
        ..default()
    });
    let mesh = meshes.add(hexagonal_column());

    commands
        .spawn((
            SpatialBundle { ..default() },
            LoadingLayer,
            Name::from("Loading"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::default().with_rotation(Quat::from_rotation_x(0.5)),
                    ..default()
                },
                Animator::new(bounce_forever),
            ));
        });
}

fn remove_loading_animation(mut commands: Commands, query: Query<(Entity,), With<LoadingLayer>>) {
    for (entity,) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Debug, Default, AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Debug, Default, AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct CatModel {
    #[asset(path = "models/cats.glb")]
    pub mesh: Handle<Gltf>,
}
