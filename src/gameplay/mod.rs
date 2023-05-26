use bevy::prelude::{shape::Cube, *};
use tracing::instrument;

use crate::{
    cats::{Cat, Meowple},
    events::{GridCellClicked, MoveCat, NewCat, NextPlayer, ResetGameEvent, WinEvent},
    grid::Map,
    players::Players,
    GameState,
};

mod boop;
mod win;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                reset_game.run_if(on_event::<ResetGameEvent>()),
                place_kitten.run_if(on_event::<GridCellClicked>()),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
        app.add_systems(
            (
                boop::plan.run_if(on_event::<NewCat>()),
                boop::move_cat.run_if(on_event::<MoveCat>()),
                win::win_condition.run_if(resource_changed::<Map>()),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );

        app.add_system(win::win_screen.run_if(on_event::<WinEvent>()));
        app.add_system(win::win_screen_cleanup.run_if(on_event::<ResetGameEvent>()));
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
