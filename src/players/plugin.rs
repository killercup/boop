use bevy::prelude::*;
use tracing::instrument;

use crate::{
    events::{NextPlayer, ResetGameEvent},
    loading::FontAssets,
    GameState,
};

use super::{Player, PlayerId, Players};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>();
        app.register_type::<Players>();

        app.add_system(setup.in_schedule(OnExit(GameState::Loading)));
        app.add_system(reset_players.run_if(on_event::<ResetGameEvent>()));

        app.add_systems((
            show_players.run_if(resource_exists_and_changed::<Players>()),
            show_current_player_indicator.run_if(resource_exists_and_changed::<Players>()),
            next_player.run_if(on_event::<NextPlayer>()),
        ));
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct PlayerInfoPanel;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct PlayerInfo(PlayerId);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct CurrentPlayerIndicator(PlayerId);

fn setup(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(30.)),
                    ..default()
                },
                ..default()
            },
            PlayerInfoPanel,
        ))
        .with_children(|panel| {
            panel
                .spawn((NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Start,
                        padding: UiRect::all(Val::Px(30.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::LIME_GREEN.with_a(0.5)),
                    ..default()
                },))
                .with_children(|player1| {
                    player1.spawn((
                        TextBundle::from_section(
                            "6 kittens",
                            TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_text_alignment(TextAlignment::Left)
                        .with_style(Style {
                            size: Size::width(Val::Px(200.)),
                            ..default()
                        }),
                        PlayerInfo(PlayerId::new(0)),
                    ));
                    player1.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::all(Val::Px(20.)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::LIME_GREEN),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        CurrentPlayerIndicator(PlayerId::new(0)),
                    ));
                });

            panel
                .spawn((NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Start,
                        padding: UiRect::all(Val::Px(30.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::ORANGE.with_a(0.5)),
                    ..default()
                },))
                .with_children(|player2| {
                    player2.spawn((
                        TextBundle::from_section(
                            "6 kittens",
                            TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_text_alignment(TextAlignment::Right)
                        .with_style(Style {
                            size: Size::width(Val::Px(200.)),
                            ..default()
                        }),
                        PlayerInfo(PlayerId::new(1)),
                    ));
                    player2.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::all(Val::Px(20.)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::ORANGE),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        CurrentPlayerIndicator(PlayerId::new(1)),
                    ));
                });
        });
}

fn show_players(players: Res<Players>, mut info: Query<(&mut Text, &PlayerInfo)>) {
    for (mut text, player) in info.iter_mut() {
        let id = player.0;
        let Player { inventory, .. } = players
            .players
            .iter()
            .find(|p| p.id == id)
            .expect("valid player id");
        *text = Text::from_section(
            format!("{} kittens", inventory.kittens),
            text.sections[0].style.clone(),
        );
    }
}

fn show_current_player_indicator(
    players: Res<Players>,
    mut indicators: Query<(&mut Visibility, &CurrentPlayerIndicator)>,
) {
    let current = players.current().id;
    for (mut vis, player) in indicators.iter_mut() {
        *vis = if player.0 == current {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn reset_players(mut players: ResMut<Players>) {
    *players = Players::default();
}

#[instrument(level = "info", skip_all)]
fn next_player(mut players: ResMut<Players>) {
    players.next_player();
}
