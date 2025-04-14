use bevy::prelude::*;

use super::{BaseCardColor, HoverCardColor};

pub struct CardColorPlugin {
    pub base: Color,
    pub hover: Color,
}

impl Plugin for CardColorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BaseCardColor(self.base))
            .insert_resource(HoverCardColor(self.hover))
            .register_type::<BaseCardColor>()
            .register_type::<HoverCardColor>();
    }
}
