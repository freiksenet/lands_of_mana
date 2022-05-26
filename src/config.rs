#[derive(Copy, Clone, Debug)]
pub struct EngineConfig {
    pub load_assets: EngineState,
    pub after_load_assets: EngineState,
    pub load_world: EngineState,
    pub after_load_world: EngineState,
    pub load_graphics: EngineState,
    pub after_load_graphics: EngineState,
    pub run_game: EngineState,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum EngineState {
    LoadingAssets,
    LoadingWorld,
    LoadingGraphics,
    InGame,
}
