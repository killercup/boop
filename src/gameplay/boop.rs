use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::{Cat, Meowple},
    events::{MoveCat, NewCat},
    grid::{GridCell, Map, MapSettings},
    loading::AudioAssets,
    players::{PlayerId, Players},
};

#[instrument(level = "debug", skip_all)]
pub fn plan(
    map: Res<Map>,
    cells_with_cats: Query<(Entity, &Cat, &PlayerId), With<GridCell>>,
    mut new_cats: EventReader<NewCat>,
    mut boops: EventWriter<MoveCat>,
) {
    for NewCat {
        cat: new_cat,
        position,
        player,
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
            let (entity, cat, player) = cells_with_cats.get(entity).ok()?;
            Some((entity, cat, player, cell, direction))
        });
        let boopable_neighbors = neighbors_with_cats
            .filter(|(_, other_cat, ..)| new_cat.can_boop(**other_cat))
            .filter(|(_, _, owner, ..)| player != *owner);

        for (boopee, _cat, _cat_owner, boopee_cell, direction) in boopable_neighbors {
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
pub fn move_cat(
    mut moves: EventReader<MoveCat>,
    settings: Res<MapSettings>,
    mut players: ResMut<Players>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut cats: Query<(Entity, &GridCell, &Transform, &PlayerId), With<Meowple>>,
    cells: Query<(&Transform,), (With<GridCell>, Without<Meowple>)>,
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
            continue;
        };

        let target_cell = map.cell_by_hex(to).unwrap();
        let (cell_position,) = cells.get(target_cell).unwrap();
        commands.entity(cat).insert(GridCell(to));

        map.add_cat(to, cat);

        let mut new_cat_position = cell_position.translation;
        new_cat_position.y += settings.column_height;

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(200),
            TransformPositionLens {
                start: cat_transform.translation,
                end: new_cat_position,
            },
        );
        commands.entity(cat).insert(Animator::new(tween));
    }
}
