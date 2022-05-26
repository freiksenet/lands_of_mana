use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::config;
pub struct AssetLoadingPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(self.config.load_assets)
            .continue_to_state(self.config.after_load_assets)
            .with_collection::<TileAssets>()
            .with_collection::<CreatureAssets>()
            // .init_resource::<CombinedTileTexture>()
            .build(app);
    }
}

// pub struct CombinedTileTexture {
//     pub texture: Handle<Image>,
//     pub size: (f32, f32),
// }

// impl FromWorld for CombinedTileTexture {
//     fn from_world(world: &mut World) -> Self {
//         let mut texture_atlas_builder = TextureAtlasBuilder::default();
//         let cell = world.cell();
//         let tile_assets = cell
//             .get_resource::<TileAssets>()
//             .expect("Failed to get ImageAssets");
//         let mut textures = cell.get_resource_mut::<Assets<Image>>().unwrap();
//         for tile_asset_handle in &tile_assets.tiles {
//             let texture = textures.get(tile_asset_handle.clone()).unwrap();

//             texture_atlas_builder.add_texture(tile_asset_handle.clone(), texture)
//         }
//         let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
//         let texture_atlas_texture = texture_atlas.texture.clone();
//         CombinedTileTexture {
//             texture: texture_atlas_texture,
//             size: (texture_atlas.size.x as f32, texture_atlas.size.y as f32),
//         }
//     }
// }

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

    #[asset(path = "ui/ui.png")]
    pub ui: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct CreatureAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 24, rows = 16,))]
    #[asset(path = "creatures/creatures.png")]
    pub creatures: Handle<TextureAtlas>,
}
