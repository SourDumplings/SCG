use crate::entity::player::{Player, PlayerSpawnParams};
use crate::entity::text::{Text, TextSpawnParams};
use crate::log_debug;
use hecs::World;

pub fn initialize_world(world: &mut World)
{
    let player_params = PlayerSpawnParams {
        x: 100.0,
        y: 100.0,
        vx: 10.0,
        vy: 10.0,
    };
    let player_entity = Player::new(world, player_params);
    log_debug!("Player entity created with ID: {:?}", player_entity);

    let text_params = TextSpawnParams {
        x: 20.0,
        y: 100.0
    };
    let text_entity = Text::new(world, text_params);
    log_debug!("Text entity created with ID: {:?}", text_entity);
}
