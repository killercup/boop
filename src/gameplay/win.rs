use bevy::{prelude::*, utils::HashMap};
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::Cat,
    events::{ResetGameEvent, WinEvent},
    grid::GridCell,
    loading::FontAssets,
    players::PlayerId,
    GameState,
};

/// A player wins if they have three adult cats in a row.
#[instrument(level = "trace", skip_all)]
pub fn win_condition(
    mut next_state: ResMut<NextState<GameState>>,
    cats: Query<(&Cat, &PlayerId, &GridCell)>,
    mut wins: EventWriter<WinEvent>,
    mut reset: EventReader<ResetGameEvent>,
) {
    // dedup
    if reset.iter().count() > 0 {
        return;
    }

    let cat_cells_by_player = cats
        .iter()
        .filter(|(cat, ..)| matches!(**cat, Cat::Kitten)) // FIXME: real cats
        .fold(
            HashMap::<PlayerId, Vec<Hex>>::new(),
            |mut map, (_cat, player, cell)| {
                map.entry(*player).or_default().push(cell.0);
                map
            },
        );

    for (player, cats) in cat_cells_by_player {
        if cats.len() < 3 {
            trace!(?player, cats = cats.len(), "not enough cats");
            continue;
        }

        for cat in &cats {
            for direction in hexx::Direction::iter() {
                let mut count = 1;
                let mut hex = cat.neighbor(direction);
                while cats.contains(&hex) {
                    count += 1;
                    hex = hex.neighbor(direction);
                }
                if count > 2 {
                    info!("Player {player} wins!");
                    wins.send(WinEvent { player });
                    next_state.set(GameState::GameOver);
                    return;
                }
                trace!(
                    ?player,
                    ?cat,
                    ?direction,
                    count,
                    "not enough cats in this row"
                );
            }
        }
    }
    trace!("no winner yet");
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct WinScreen;

#[instrument(level = "debug", skip_all)]
pub fn win_screen(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    mut event: EventReader<WinEvent>,
) {
    let event = event.iter().next().unwrap();
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    gap: Size::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE.with_a(0.6)),
                ..default()
            },
            WinScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                format!("Player {} won!", event.player),
                TextStyle {
                    font: fonts.fira_sans.clone(),
                    font_size: 48.0,
                    color: Color::BLACK,
                },
            ),));
            parent.spawn((TextBundle::from_section(
                "Press R to restart",
                TextStyle {
                    font: fonts.fira_sans.clone(),
                    font_size: 18.0,
                    color: Color::DARK_GRAY,
                },
            ),));
        });
}

pub fn win_screen_cleanup(mut commands: Commands, query: Query<Entity, With<WinScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
