use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum WorldActions {
    Pause,
    Resume,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SelectActions {
    Select,
    Deselect,
    HoverSelectable,
}
