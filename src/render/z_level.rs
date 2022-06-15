pub enum ZLevel {
    Base = 1,
    Connectors1 = 2,
    Connectors2 = 3,
    Connectors3 = 4,
    Connectors4 = 5,
    Sites = 7,
    Borders = 10,
    Background = 50,
    Units = 75,
    UnitDecorations = 80,
}

impl From<ZLevel> for f32 {
    fn from(z_level: ZLevel) -> Self {
        z_level as usize as f32
    }
}
