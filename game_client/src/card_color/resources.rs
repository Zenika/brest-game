use bevy::prelude::*;

pub trait CardColor {
    fn as_color(&self) -> Color;
}

#[derive(Resource, Reflect)]
pub struct BaseCardColor(pub Color);

impl CardColor for BaseCardColor {
    fn as_color(&self) -> Color {
        self.0
    }
}

#[derive(Resource, Reflect)]
pub struct HoverCardColor(pub Color);

impl CardColor for HoverCardColor {
    fn as_color(&self) -> Color {
        self.0
    }
}
