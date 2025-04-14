use bevy::prelude::*;

use super::{BaseCardMaterial, HoverCardMaterial};

pub struct CardMaterialPlugin;

impl Plugin for CardMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BaseCardMaterial>()
            .init_resource::<HoverCardMaterial>()
            .register_type::<BaseCardMaterial>()
            .register_type::<HoverCardMaterial>();
    }
}
