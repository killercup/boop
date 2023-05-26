use std::time::Duration;

use bevy::prelude::{shape::Cube, *};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::{Cat, Meowple},
    grid::{events::GridCellClicked, GridCell, Map, MapSettings},
    players::{events::NextPlayer, PlayerId, Players},
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
    map: Res<Map>,
    mut places: EventReader<GridCellClicked>,
    mut players: ResMut<Players>,
    mut new_cat: EventWriter<NewCat>,
    mut next_player: EventWriter<NextPlayer>,
) {
    if places.len() > 1 {
        error!("More than one place clicked, ignoring all but first");
    }
    let ev = places.iter().next().unwrap();

    let Some(hex) = map.cell_by_entity(ev.cell) else {
        warn!("Place not on map");
        return;
    };

    if map.cat_by_hex(hex).is_some() {
        warn!("Cannot put cat where there's already a cat");
        return;
    };

    let player = players.current().id;
    let Some(new_kitten) = players.take_kitten() else {
        warn!("No more kittens to place");
        return;
    };

    new_cat.send(NewCat {
        player,
        cat: new_kitten,
        cell: ev.cell,
        position: hex,
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
        ..
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
            let entity = map.cat_by_hex(cell)?;
            let components = cells_with_cats.get(entity).ok()?;
            Some((components, cell, direction))
        });
        let boopable_neighbors =
            neighbors_with_cats.filter(|((.., other_cat), ..)| new_cat.can_boop(**other_cat));

        for ((boopee, cat), boopee_cell, direction) in boopable_neighbors {
            let possible_boop_destination = boopee_cell + direction;

            match map.cell_by_hex(possible_boop_destination) {
                Some(_) => {
                    if map.cat_by_hex(possible_boop_destination).is_some() {
                        debug!(?possible_boop_destination, "Cannot boop to cell with cat");
                        continue;
                    }
                    boops.send(MoveCat {
                        from: boopee,
                        to: Some(possible_boop_destination),
                    });
                }
                None => {
                    boops.send(MoveCat {
                        from: boopee,
                        to: None,
                    });
                }
            };
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn move_cat(
    settings: Res<MapSettings>,
    mut moves: EventReader<MoveCat>,
    mut players: ResMut<Players>,
    mut commands: Commands,
    mut map: ResMut<Map>,
    cells: Query<(&Transform,), (With<GridCell>, Without<Meowple>)>,
    mut cats: Query<(Entity, &GridCell, &Transform, &PlayerId), With<Meowple>>,
) {
    for MoveCat { from, to } in moves.iter() {
        debug!(?from, ?to, "Moving cat");
        let (cat, cat_cell, cat_transform, player_id) = match cats.get_mut(*from) {
            Ok(x) => x,
            Err(error) => {
                error!(entity=?from, ?error, "Cell with cat not found");
                continue;
            }
        };

        map.clear_cat_cell(cat_cell.0);

        let Some(to) = *to else {
            debug!(?from, ?to, "Bye bye cat");
            commands.entity(*from).despawn_recursive();
            players.gain_kitten(*player_id);
            return;
        };

        let target_cell = map.cell_by_hex(to).unwrap();
        let (cell_position,) = cells.get(target_cell).unwrap();
        commands.entity(cat).insert(GridCell(to));

        map.add_cat(to, cat);

        let mut new_cat_position = cell_position.translation;
        new_cat_position.y += settings.column_height;

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(260),
            TransformPositionLens {
                start: cat_transform.translation,
                end: new_cat_position,
            },
        );
        commands.entity(cat).insert(Animator::new(tween));
    }
}
