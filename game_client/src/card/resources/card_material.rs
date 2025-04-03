use bevy::prelude::*;

use super::{BaseCardColor, CardColor, HoverCardColor};

pub trait CardMaterial {
    fn as_material(&self) -> Handle<StandardMaterial>;
}

#[derive(Resource, Reflect)]
pub struct BaseCardMaterial(Handle<StandardMaterial>);

impl CardMaterial for BaseCardMaterial {
    fn as_material(&self) -> Handle<StandardMaterial> {
        self.0.clone()
    }
}

impl FromWorld for BaseCardMaterial {
    fn from_world(world: &mut World) -> Self {
        let color = world.resource::<BaseCardColor>().as_color();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        BaseCardMaterial(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        }))
    }
}

#[derive(Resource, Reflect)]
pub struct HoverCardMaterial(Handle<StandardMaterial>);

impl CardMaterial for HoverCardMaterial {
    fn as_material(&self) -> Handle<StandardMaterial> {
        self.0.clone()
    }
}

impl FromWorld for HoverCardMaterial {
    fn from_world(world: &mut World) -> Self {
        let color = world.resource::<HoverCardColor>().as_color();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        HoverCardMaterial(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        }))
    }
}
