use bevy::{prelude::*, ui::FocusPolicy, utils::HashMap};
use bevy_mod_picking::prelude::*;
use hexx::Hex;
use tracing::instrument;

use crate::{
    cats::Cat,
    events::{ResetGameEvent, WinEvent},
    grid::GridCell,
    loading::FontAssets,
    players::{PlayerId, Players},
    GameState,
};

#[instrument(level = "trace", skip_all)]
pub fn draw_condition(
    players: Res<Players>,
    mut next_state: ResMut<NextState<GameState>>,
    mut reset: EventReader<ResetGameEvent>,
    mut wins: EventWriter<WinEvent>,
) {
    // dedup
    if reset.iter().count() > 0 {
        return;
    }
    if !players.players.iter().any(|player| player.can_do_turn()) {
        wins.send(WinEvent { winner: None });
        next_state.set(GameState::GameOver);
    }
}

/// A player wins if they have three adult cats in a row.
#[instrument(level = "trace", skip_all)]
pub fn win_condition(
    mut next_state: ResMut<NextState<GameState>>,
    mut reset: EventReader<ResetGameEvent>,
    mut wins: EventWriter<WinEvent>,
    cats: Query<(&Cat, &PlayerId, &GridCell)>,
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
                    wins.send(WinEvent {
                        winner: Some(player),
                    });
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
    fonts: Res<FontAssets>,
    players: Res<Players>,
    mut commands: Commands,
    mut event: EventReader<WinEvent>,
    previous_win_screens: Query<(Entity,), With<WinScreen>>,
) {
    let event = event.iter().next().unwrap();

    previous_win_screens.iter().for_each(|(entity,)| {
        commands.entity(entity).despawn_recursive();
    });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(20.)),
                    gap: Size::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE.with_a(0.6)),
                ..default()
            },
            WinScreen,
        ))
        .with_children(|parent| {
            match event.winner {
                Some(id) => {
                    let player = players.by_id(id).expect("valid player id");
                    parent.spawn((TextBundle::from_section(
                        format!("{} won!", player.name),
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    ),));
                }
                None => {
                    parent.spawn((TextBundle::from_section(
                        "Tie!",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    ),));
                    parent.spawn((TextBundle::from_section(
                        "No more cats!",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    ),));
                }
            };
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE.with_a(0.5)),
                        ..default()
                    },
                    OnPointer::<Over>::target_insert(BackgroundColor::from(
                        Color::WHITE.with_a(0.8),
                    )),
                    OnPointer::<Click>::send_event::<ResetGameEvent>(),
                ))
                .with_children(|button| {
                    let mut text = TextBundle::from_section(
                        "Press R to restart",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 18.0,
                            color: Color::DARK_GRAY,
                        },
                    );
                    text.focus_policy = FocusPolicy::Pass;
                    button.spawn((text,));
                });
        });
}

impl From<ListenedEvent<Click>> for ResetGameEvent {
    fn from(_event: ListenedEvent<Click>) -> Self {
        ResetGameEvent
    }
}

pub fn win_screen_cleanup(mut commands: Commands, query: Query<Entity, With<WinScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
