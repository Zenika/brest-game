use bevy::prelude::*;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct BattleRoundCount(pub u8);

impl FromWorld for BattleRoundCount {
    fn from_world(_world: &mut World) -> Self {
        BattleRoundCount(0)
    }
}
