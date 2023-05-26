use bevy::{prelude::*, utils::HashMap};
use hexx::Hex;

use crate::{
    cats::Cat, events::WinEvent, grid::GridCell, loading::FontAssets, players::PlayerId, GameState,
};

/// A player wins if they have three adult cats in a row.
pub fn win_condition(
    mut next_state: ResMut<NextState<GameState>>,
    cats: Query<(&Cat, &PlayerId, &GridCell)>,
    mut wins: EventWriter<WinEvent>,
) {
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
            continue;
        }

        for cat in &cats {
            let mut count = 0;
            for direction in hexx::Direction::iter() {
                let mut hex = *cat;
                while cats.contains(&hex.neighbor(direction)) {
                    count += 1;
                    hex = hex.neighbor(direction);
                }
            }
            if count >= 2 {
                info!("Player {player} wins!");
                wins.send(WinEvent { player });
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct WinScreen;

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
        });
}

pub fn win_screen_cleanup(mut commands: Commands, query: Query<Entity, With<WinScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}