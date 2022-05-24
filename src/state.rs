#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    LoadingAssets,
    LoadingWorld,
    InGame,
}
