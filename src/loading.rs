use crate::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<SummonsAssets>(),
        )
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<SummonType>::new(
            &["summon"],
        ));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 8))]
    pub board_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "Tiles.png")]
    pub board: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SummonsAssets {
    #[asset(path = "summons/angel.summon")]
    pub angel: Handle<SummonType>,
    #[asset(path = "summons/skeleton.summon")]
    pub skeleton: Handle<SummonType>,
    #[asset(path = "summons/watcher.summon")]
    pub watcher: Handle<SummonType>,
}
