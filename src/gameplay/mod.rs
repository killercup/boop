use bevy::prelude::{shape::Cube, *};
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::Cat,
    grid::{events::GridCellClicked, GridCell, Map},
    player::{CurrentPlayer, Player},
};

use self::events::{MoveCat, NewCat, ResetGameEvent};

pub mod events;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetGameEvent>();
        app.add_event::<NewCat>();
        app.add_event::<MoveCat>();

        app.add_startup_system(setup);
        app.add_system(reset_game.run_if(on_event::<events::ResetGameEvent>()));
        app.add_system(place_kitten.run_if(on_event::<GridCellClicked>()));
        app.add_system(boop_plan.run_if(on_event::<NewCat>()));
        app.add_system(move_cat.run_if(on_event::<MoveCat>()));
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct KittenMaterials {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// Cat figurine
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Meowple;

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
fn reset_game(
    mut commands: Commands,
    meowple: Query<(Entity,), With<Meowple>>,
    mut cat_cells: Query<(Entity,), With<Cat>>,
) {
    for (cat,) in meowple.iter() {
        commands.entity(cat).despawn_recursive();
    }

    for (cell,) in cat_cells.iter_mut() {
        commands.entity(cell).remove::<Cat>();
    }
}

#[instrument(level = "debug", skip_all)]
fn place_kitten(
    mut places: EventReader<GridCellClicked>,
    mut commands: Commands,
    mut players: Query<(&mut Player,)>,
    current_player: Res<CurrentPlayer>,
    mut new_cat: EventWriter<NewCat>,
    // map: Res<Map>,
    // cells_with_cats: Query<(Entity, &Cat), With<GridCell>>,
    cells_without_cats: Query<(Entity,), (With<GridCell>, Without<Cat>)>,
    kitten: Res<KittenMaterials>,
) {
    let Ok((mut player,)) = players.get_mut(current_player.id) else{
        error!("Current player not found");
        return;
    };

    if places.len() > 1 {
        error!("More than one place clicked, ignoring all but first");
    }
    let ev = places.iter().next().unwrap();

    if !cells_without_cats.contains(ev.cell) {
        warn!("Cell already has a cat, ignoring");
        return;
    }

    let Some(new_kitten) = player.take_kitten() else {
        warn!("No more kittens to place");
        return;
    };

    commands
        .entity(ev.cell)
        .insert(new_kitten)
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: kitten.mesh.clone(),
                    material: kitten.material.clone(),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                },
                Name::from("Kitten"),
                Meowple,
            ));
        });

    new_cat.send(NewCat {
        cat: new_kitten,
        cell: ev.cell,
    });
}

#[instrument(level = "debug", skip_all)]
fn boop_plan(
    map: Res<Map>,
    cells_with_cats: Query<(Entity, &Cat), With<GridCell>>,
    mut new_cats: EventReader<NewCat>,
    mut boops: EventWriter<MoveCat>,
) {
    for NewCat { cat: new_cat, cell } in new_cats.iter() {
        let Some(cell) = map.cell_by_entity(*cell) else {
            error!(entity=?cell, "Cell not found");
            return;
        };

        let boopees = cell.all_neighbors();
        let boopees = boopees
            .iter()
            .zip(Hex::NEIGHBORS_COORDS)
            .filter_map(|(&cell, direction)| {
                let entity = map.cell_by_hex(cell)?;
                let components = cells_with_cats.get(entity).ok()?;
                Some((components, direction))
            })
            .filter(|((.., other_cat), _direction)| new_cat.can_boop(**other_cat));

        for ((boopee, cat), direction) in boopees {
            let Some(boopee_cell) = map.cell_by_entity(boopee) else {
                debug!(?boopee, "Cannot find cell for entity");
                continue;
            };
            let destination = boopee_cell + direction;
            let Some(dest_cell) = map.cell_by_hex(destination) else {
                debug!(?destination, "Cannot boop outside map");
                continue;
            };
            if !cells_with_cats.contains(dest_cell) {
                debug!(?destination, "Cannot boop to cell with cat");
            }
            boops.send(MoveCat {
                from: boopee,
                to: dest_cell,
            });
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn move_cat(
    mut moves: EventReader<MoveCat>,
    mut commands: Commands,
    mut source_cells: Query<(&Children,), (With<GridCell>)>,
    mut target_cells: Query<(Entity,), (With<GridCell>)>,
    cats: Query<(Entity, &mut Parent), With<Meowple>>,
) {
    for MoveCat { from, to } in moves.iter() {
        debug!(?from, ?to, "Moving cat");
        let (children,) = match source_cells.get_mut(*from) {
            Ok(x) => x,
            Err(error) => {
                error!(entity=?from, ?error, "Cell with cat to not found");
                continue;
            }
        };
        let (new_parent,) = match target_cells.get_mut(*to) {
            Ok(x) => x,
            Err(error) => {
                error!(entity=?from, ?error, "Cell to move cat to not found, is it empty?");
                continue;
            }
        };
        for child in children {
            if cats.contains(*child) {
                commands.entity(*child).set_parent(new_parent);
                // commands.entity(new_parent).add_child(*child);
            } else {
                error!(entity=?child, "Child is not a cat");
            }
        }
    }
}
