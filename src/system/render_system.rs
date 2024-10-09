use crate::component::PositionComponent;
use crate::resource::SpriteManager;
use hecs::World;
use macroquad::prelude::*;

pub fn render_system(world: &World, sprite_manager: &SpriteManager)
{
    for (_, pos) in world.query::<&PositionComponent>().iter()
    {
        if let Some(texture) = sprite_manager.get_texture("mushroom")
        {
            draw_texture(texture, pos.x, pos.y, WHITE);
        }
        else
        {
            draw_circle(pos.x, pos.y, 10.0, WHITE);
        }
        draw_text(
            &format!("Position: ({:.2}, {:.2})", pos.x, pos.y),
            20.0,
            20.0,
            30.0,
            WHITE,
        );
    }
}
