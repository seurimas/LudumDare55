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
                .load_collection::<StyleAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<SummonsAssets>()
                .load_collection::<BrainAssets>(),
        )
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<SummonType>::new(
            &["summon"],
        ))
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<Story>::new(&[
            "teller",
        ]))
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<SummonedMinions>::new(&["wave"]))
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<CharacterBrainDef>::new(&["brain"]));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(
        paths(
            "audio/angel_summon_sting.wav",
            "audio/construct_summon_sting.wav",
            "audio/demon_summon_sting.wav",
            "audio/elemental_summon_sting.wav",
            "audio/fairy_summon_sting.wav",
            "audio/undead_summon_sting.wav",
            "audio/enemy_summon_sting.wav"
        ),
        collection(typed, mapped)
    )]
    pub summon_stings: HashMap<FileStem, Handle<AudioSource>>,
    #[asset(path = "audio/defeat_sting.wav")]
    pub defeat_sting: Handle<AudioSource>,
    #[asset(path = "audio/victory_sting.wav")]
    pub victory_sting: Handle<AudioSource>,
    #[asset(path = "audio/hurt.wav")]
    pub hurt: Handle<AudioSource>,
    #[asset(path = "audio/place.wav")]
    pub place: Handle<AudioSource>,
    #[asset(path = "audio/remove.wav")]
    pub remove: Handle<AudioSource>,
    #[asset(path = "audio/type.wav")]
    pub type_char: Handle<AudioSource>,
    #[asset(path = "audio/error.wav")]
    pub error: Handle<AudioSource>,
    #[asset(path = "audio/game_over_victory.wav")]
    pub game_over_victory: Handle<AudioSource>,
    #[asset(path = "audio/game_over_defeat.wav")]
    pub game_over_defeat: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct StyleAssets {
    #[asset(path = "sheets/loot.css")]
    pub loot: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/main_menu.css")]
    pub main_menu: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/summon_button.css")]
    pub summon_button: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/summon_scroll.css")]
    pub summon_scroll: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/narration.css")]
    pub narration: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/game_over.css")]
    pub game_over: Handle<StyleSheetAsset>,
    #[asset(path = "sheets/help.css")]
    pub help: Handle<StyleSheetAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 8))]
    pub board_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "Tiles.png")]
    pub board: Handle<Image>,
    #[asset(path = "Summon.png")]
    pub summon: Handle<Image>,
    #[asset(path = "ScrollBack.png")]
    pub scroll_back: Handle<Image>,
    #[asset(path = "ScrollSide.png")]
    pub scroll_side: Handle<Image>,
    #[asset(path = "Narration.png")]
    pub narration: Handle<Image>,
    #[asset(path = "Background.png")]
    pub background: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SummonsAssets {
    #[asset(path = "summons", collection(typed, mapped))]
    pub player_summons: HashMap<FileStem, Handle<SummonType>>,
    #[asset(path = "npc", collection(typed, mapped))]
    pub npc_summons: HashMap<FileStem, Handle<SummonType>>,
    #[asset(path = "waves", collection(typed, mapped))]
    pub waves: HashMap<FileStem, Handle<SummonedMinions>>,
    #[asset(path = "story.teller")]
    pub story_teller: Handle<Story>,
}

#[derive(AssetCollection, Resource)]
pub struct BrainAssets {
    #[asset(path = "brains", collection(typed, mapped))]
    pub brains: HashMap<FileStem, Handle<CharacterBrainDef>>,
}
