use crate::GameState;
use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        );
        app.add_collection_to_loading_state::<_, FontAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, CatModel>(GameState::Loading);
    }
}

#[derive(Debug, Default, AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Debug, Default, AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct CatModel {
    #[asset(path = "models/cats.glb")]
    pub mesh: Handle<Gltf>,
}
