use bevy::prelude::{shape::Cube, *};
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::{Cat, Meowple},
    grid::{events::GridCellClicked, GridCell, Map},
    players::{events::NextPlayer, Players},
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
    material_player1: Handle<StandardMaterial>,
    material_player2: Handle<StandardMaterial>,
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cube::default().into());

    commands.insert_resource(KittenMaterials {
        mesh,
        material_player1: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        material_player2: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..default()
        }),
    });
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
    mut players: ResMut<Players>,
    mut new_cat: EventWriter<NewCat>,
    mut next_player: EventWriter<NextPlayer>,
    cells_without_cats: Query<(&GridCell,), Without<Cat>>,
) {
    if places.len() > 1 {
        error!("More than one place clicked, ignoring all but first");
    }
    let ev = places.iter().next().unwrap();

    let Some(new_kitten) = players.take_kitten() else {
        warn!("No more kittens to place");
        return;
    };

    let Ok((grid_cell,)) = cells_without_cats.get(ev.cell) else {
        warn!(cell=?ev.cell, "Cell already has a cat, ignoring");
        return;
    };

    new_cat.send(NewCat {
        cat: new_kitten,
        cell: ev.cell,
        position: **grid_cell,
    });

    next_player.send(NextPlayer);
}

#[instrument(level = "debug", skip_all)]
fn boop_plan(
    map: Res<Map>,
    cells_with_cats: Query<(Entity, &Cat), With<GridCell>>,
    mut new_cats: EventReader<NewCat>,
    mut boops: EventWriter<MoveCat>,
) {
    for NewCat {
        cat: new_cat,
        cell,
        position,
    } in new_cats.iter()
    {
        // find all neighbors
        // filter out those that are not on the map
        // filter out those that do not have a cat
        // filter out those that are not boopable
        // for each of those, find the cell in the direction of the boop
        // if that cell is not on the map, cat disappears
        // else if that cell has a cat, ignore
        // else move cat to that cell

        let neighbors = Hex::NEIGHBORS_COORDS
            .into_iter()
            .map(|direction| (*position + direction, direction));
        let neighbors_with_cats = neighbors.filter_map(|(cell, direction)| {
            let entity = map.cell_by_hex(cell)?;
            let components = cells_with_cats.get(entity).ok()?;
            Some((components, cell, direction))
        });
        let boopable_neighbors =
            neighbors_with_cats.filter(|((.., other_cat), ..)| new_cat.can_boop(**other_cat));

        for ((boopee, cat), boopee_cell, direction) in boopable_neighbors {
            let possible_boop_destination = boopee_cell + direction;

            let Some(dest_cell) = map.cell_by_hex(possible_boop_destination) else {
                boops.send(MoveCat {
                    from: boopee,
                    to: None,
                });
                continue;
            };
            if !cells_with_cats.contains(dest_cell) {
                debug!(?possible_boop_destination, "Cannot boop to cell with cat");
            }
            boops.send(MoveCat {
                from: boopee,
                to: Some(dest_cell),
            });
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn move_cat(
    mut moves: EventReader<MoveCat>,
    mut commands: Commands,
    mut source_cells: Query<(Entity, &Children), With<GridCell>>,
    mut target_cells: Query<(Entity,), With<GridCell>>,
    cats: Query<(Entity, &mut Parent), With<Meowple>>,
) {
    for MoveCat { from, to } in moves.iter() {
        debug!(?from, ?to, "Moving cat");
        let (source_cell, children) = match source_cells.get_mut(*from) {
            Ok(x) => x,
            Err(error) => {
                error!(entity=?from, ?error, "Cell with cat to not found");
                continue;
            }
        };

        commands.entity(source_cell).remove::<Cat>();

        let Some(to) = *to else {
            debug!(?from, ?to, "Bye bye cat");
            commands.entity(*from).despawn_descendants();
            return;
        };

        let (new_parent,) = match target_cells.get_mut(to) {
            Ok(x) => x,
            Err(error) => {
                error!(entity=?from, ?error, "Cell to move cat to not found, is it empty?");
                continue;
            }
        };
        for child in children {
            if cats.contains(*child) {
                commands.entity(*child).set_parent(new_parent);
            } else {
                error!(entity=?child, "Child is not a cat");
            }
        }
    }
}
