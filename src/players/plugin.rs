use bevy::prelude::*;

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

        app.add_systems(
            (
                show_players.run_if(resource_changed::<Players>()),
                reset_players.run_if(on_event::<ResetGameEvent>()),
                next_player.run_if(on_event::<NextPlayer>()),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct PlayerInfoPanel;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct PlayerInfo(PlayerId);

fn setup(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Px(160.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    gap: Size::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            PlayerInfoPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                PlayerInfo(PlayerId::new(0)),
            ));

            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::Left),
                PlayerInfo(PlayerId::new(1)),
            ));
        });
}

fn show_players(players: Res<Players>, mut info: Query<(&mut Text, &PlayerInfo)>) {
    for (mut text, player) in info.iter_mut() {
        let id = player.0;
        let Player { id, inventory } = players
            .players
            .iter()
            .find(|p| p.id == id)
            .expect("valid player id");
        *text = Text::from_section(
            format!("{id:?}\n{inventory:#?}"),
            text.sections[0].style.clone(),
        );
    }
}

fn reset_players(mut players: ResMut<Players>) {
    *players = Players::default();
}

fn next_player(mut players: ResMut<Players>) {
    players.next_player();
}
