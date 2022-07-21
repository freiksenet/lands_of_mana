pub enum ZLevel {
    Background = 1,
    Base = 5,
    Connectors1 = 10,
    Connectors2 = 11,
    Connectors3 = 12,
    Connectors4 = 13,
    Rivers = 20,
    Roads = 21,
    Mountains = 22,
    Forests = 23,
    Topology = 24,
    Decorations = 25,
    Sites = 26,
    Borders = 30,
    Units = 75,
    UnitDecorations = 80,
    OrderDirections = 81,
}

impl From<ZLevel> for f32 {
    fn from(z_level: ZLevel) -> Self {
        z_level as usize as f32
    }
}
