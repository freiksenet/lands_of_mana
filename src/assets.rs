use bevy_asset_loader::{AssetCollection, AssetLoader};
use kayak_ui::font::KayakFont;

use crate::prelude::*;
pub struct AssetLoadingPlugin {}

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(config::EngineState::LoadingAssets)
            .continue_to_state(config::EngineState::LoadingAssets.next())
            .with_collection::<TileAssets>()
            .with_collection::<CreatureAssets>()
            .with_collection::<UiAssets>()
            .with_collection::<FontAssets>()
            .with_collection::<IconAssets>()
            // .init_resource::<CombinedTileTexture>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct TileAssets {
    // #[asset(path = "tiles", folder(typed))]
    // tiles: Vec<Handle<Image>>,
    #[asset(path = "tiles/terrain_base.png")]
    pub terrain_base: Handle<Image>,

    #[asset(path = "tiles/terrain_connectors.png")]
    pub terrain_connectors: Handle<Image>,

    #[asset(path = "tiles/fog_of_war_and_map.png")]
    pub fog_of_war_and_map: Handle<Image>,

    #[asset(path = "sites/sites.png")]
    pub sites: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct CreatureAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 24, rows = 16,))]
    #[asset(path = "creatures/creatures.png")]
    pub creatures: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(path = "ui/buttons/pause", collection(typed))]
    pub button_pause: Vec<Handle<Image>>,

    #[asset(path = "ui/buttons/resume", collection(typed))]
    pub button_resume: Vec<Handle<Image>>,

    #[asset(path = "ui/bg/ui_window_light.png")]
    pub window_light: Handle<Image>,
    #[asset(path = "ui/bg/ui_window_light_top.png")]
    pub window_light_top: Handle<Image>,

    #[asset(path = "ui/bg/ui_window_cornered.png")]
    pub window_cornered: Handle<Image>,

    #[asset(path = "ui/bg/ui_window_scroll.png")]
    pub window_scroll: Handle<Image>,

    #[asset(path = "ui/bg/ui_window_paper_top.png")]
    pub window_paper_top: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct IconAssets {
    #[asset(path = "ui/icons/mana-arcana.png")]
    pub mana_arcana: Handle<Image>,
    #[asset(path = "ui/icons/mana-chaos.png")]
    pub mana_chaos: Handle<Image>,
    #[asset(path = "ui/icons/mana-death.png")]
    pub mana_death: Handle<Image>,
    #[asset(path = "ui/icons/mana-nature.png")]
    pub mana_nature: Handle<Image>,
    #[asset(path = "ui/icons/mana-sun.png")]
    pub mana_sun: Handle<Image>,

    #[asset(path = "ui/icons/res-gold.png")]
    pub res_gold: Handle<Image>,
    #[asset(path = "ui/icons/res-wood.png")]
    pub res_wood: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/compass.kayak_font")]
    pub compass: Handle<KayakFont>,
}
