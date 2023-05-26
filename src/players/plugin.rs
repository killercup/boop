use bevy::prelude::*;

use crate::events::{NextPlayer, ResetGameEvent};

use super::{Player, PlayerId, Players};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>();
        app.register_type::<Players>();

        app.add_startup_system(setup);
        app.add_system(show_players.run_if(resource_changed::<Players>()));
        app.add_system(reset_players.run_if(on_event::<ResetGameEvent>()));
        app.add_system(next_player.run_if(on_event::<NextPlayer>()));
    }
}

#[derive(Debug, Component)]
struct PlayerInfo(PlayerId);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Px(160.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                gap: Size::all(Val::Px(10.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
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
                        font,
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
