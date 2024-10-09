use crate::component::PositionComponent;
use crate::resource_manager::ResourceManager;
use hecs::World;
use macroquad::prelude::*;

pub fn render_system(world: &World, resource_manager: &ResourceManager)
{
    for (_, pos) in world.query::<&PositionComponent>().iter()
    {
        if let Some(texture) = resource_manager.get_texture("mushroom")
        {
            draw_texture(&texture, pos.x, pos.y, WHITE);
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

        resource_manager
            .fonts
            .draw_text("你好，世界！", 20.0, 100.0, 30, WHITE);
    }
}
