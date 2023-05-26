use bevy::prelude::{shape::Cube, *};
use tracing::instrument;

use crate::{
    cats::Cat,
    grid::{events::GridCellClicked, GridCell, Map},
    player::{CurrentPlayer, Player},
};

pub mod events;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::ResetGameEvent>();

        app.add_startup_system(setup);
        app.add_system(reset_game.run_if(on_event::<events::ResetGameEvent>()));
        app.add_system(place_kitten.run_if(on_event::<GridCellClicked>()));
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct KittenMaterials {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cube::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..default()
    });

    commands.insert_resource(KittenMaterials { mesh, material });
}

#[instrument(level = "info", skip_all)]
fn reset_game(mut commands: Commands, cats: Query<(Entity,), With<Cat>>) {
    for (cat,) in cats.iter() {
        commands.entity(cat).despawn_recursive();
    }
}

fn place_kitten(
    mut places: EventReader<GridCellClicked>,
    mut commands: Commands,
    mut players: Query<(&mut Player,)>,
    map: Res<Map>,
    cells_with_cats: Query<(Entity, &GridCell, &Cat)>,
    kitten: Res<KittenMaterials>,
    current_player: Res<CurrentPlayer>,
) {
    let Ok((mut player,)) = players.get_mut(current_player.id) else{
        error!("Current player not found");
        return;
    };
    let Some(new_kitten) = player.take_kitten() else {
        warn!("No more kittens to place");
        return;
    };

    if places.len() > 1 {
        error!("More than one place clicked, ignoring all but first");
    }
    let ev = places.iter().next().unwrap();

    commands.entity(ev.cell).with_children(|parent| {
        parent.spawn((
            PbrBundle {
                mesh: kitten.mesh.clone(),
                material: kitten.material.clone(),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Name::from("Kitten"),
            new_kitten,
        ));
    });

    let Some(cell) = map.cell_by_entity(ev.cell) else {
        error!(entity=?ev.cell, "Cell not found");
        return;
    };
}
