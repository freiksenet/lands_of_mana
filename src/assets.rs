use bevy_asset_loader::prelude::*;

use crate::prelude::*;
pub struct AssetLoadingPlugin {}

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(config::EngineState::LoadingAssets)
                .continue_to_state(config::EngineState::LoadingAssets.next())
                .with_collection::<TileAssets>()
                .with_collection::<CreatureAssets>()
                .with_collection::<UiAssets>()
                // .with_collection::<FontAssets>()
                .with_collection::<IconAssets>(),
        );
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

    #[asset(path = "tiles/decorations.png")]
    pub decorations: Handle<Image>,

    #[asset(path = "tiles/forest_and_mountains.png")]
    pub forest_and_mountains: Handle<Image>,

    #[asset(path = "tiles/roads_and_rivers.png")]
    pub roads_and_rivers: Handle<Image>,

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
    #[asset(path = "ui/buttons", collection, typed)]
    pub buttons: Vec<Handle<Image>>,

    #[asset(path = "ui/windows", collection, typed)]
    pub windows: Vec<Handle<Image>>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 8, rows = 10))]
    #[asset(path = "ui/selectors.png")]
    pub selectors: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 16))]
    #[asset(path = "ui/directions.png")]
    pub directions: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 8))]
    #[asset(path = "ui/cursors.png")]
    pub cursors: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 12))]
    #[asset(path = "ui/clicks.png")]
    pub clicks: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 24., columns = 8, rows = 2))]
    #[asset(path = "ui/badges.png")]
    pub badges: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 12., tile_size_y = 18., columns = 2, rows = 5))]
    #[asset(path = "ui/unitbadges.png")]
    pub unit_badges: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 2))]
    #[asset(path = "ui/unit_badge_icons.png")]
    pub unit_badge_icons: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct IconAssets {
    #[asset(path = "icons/outline", collection, typed)]
    pub outline: Vec<Handle<Image>>,
}
