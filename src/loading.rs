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
    pub summon_stings: HashMap<AssetFileStem, Handle<AudioSource>>,
    #[asset(
        paths(
            "audio/angel_death_sting.wav",
            "audio/construct_death_sting.wav",
            "audio/demon_death_sting.wav",
            "audio/elemental_death_sting.wav",
            "audio/fairy_death_sting.wav",
            "audio/undead_death_sting.wav",
            "audio/enemy_death_sting.wav"
        ),
        collection(typed, mapped)
    )]
    pub death_stings: HashMap<AssetFileStem, Handle<AudioSource>>,
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
    #[asset(path = "audio/welcome.wav")]
    pub welcome: Handle<AudioSource>,
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
    #[asset(path = "Title.png")]
    pub title: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SummonsAssets {
    #[asset(
        paths(
            "summons/Bane.summon",
            "summons/Brutalizer.summon",
            "summons/Butcher.summon",
            "summons/Cherub.summon",
            "summons/Ember.summon",
            "summons/Feline.summon",
            "summons/Ghost.summon",
            "summons/Ghoul.summon",
            "summons/Golem.summon",
            "summons/Guardian.summon",
            "summons/Pixie.summon",
            "summons/Pylon.summon",
            "summons/Seraph.summon",
            "summons/Skeleton.summon",
            "summons/Undine.summon",
            "summons/Vampire.summon",
            "summons/Virtue.summon",
            "summons/Vulpine.summon",
            "summons/Watcher.summon",
            "summons/Wisp.summon",
            "summons/Wolfine.summon",
        ),
        collection(typed, mapped)
    )]
    pub player_summons: HashMap<AssetFileStem, Handle<SummonType>>,
    #[asset(
        paths(
            "npc/Bones.summon",
            "npc/Death.summon",
            "npc/Pain.summon",
            "npc/Necromancer.summon",
        ),
        collection(typed, mapped)
    )]
    pub npc_summons: HashMap<AssetFileStem, Handle<SummonType>>,
    #[asset(
        paths(
            "waves/wave0.wave",
            "waves/wave1.wave",
            "waves/wave2.wave",
            "waves/wave3.wave",
            "waves/wave4.wave",
            "waves/wave5.wave",
            "waves/wave6.wave",
            "waves/wave7.wave",
            "waves/wave8.wave",
            "waves/boss0.wave",
            "waves/boss1.wave",
        ),
        collection(typed, mapped)
    )]
    pub waves: HashMap<AssetFileStem, Handle<SummonedMinions>>,
    #[asset(path = "story.teller")]
    pub story_teller: Handle<Story>,
}

#[derive(AssetCollection, Resource)]
pub struct BrainAssets {
    #[asset(
        paths(
            "brains/construct.brain",
            "brains/demon.brain",
            "brains/death.brain",
            "brains/draining.brain",
            "brains/elemental_buff_death.brain",
            "brains/evading_debuff_nearest.brain",
            "brains/evading_prioritized.brain",
            "brains/evading.brain",
            "brains/fighter_prioritized.brain",
            "brains/fighter.brain",
            "brains/large_construct.brain",
            "brains/necromancer.brain",
        ),
        collection(typed, mapped)
    )]
    pub brains: HashMap<AssetFileStem, Handle<CharacterBrainDef>>,
}
