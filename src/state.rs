#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    LoadingAssets,
    LoadingWorld,
    LoadingGraphics,
    InGame,
}
