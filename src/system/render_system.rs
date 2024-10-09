use crate::component::{PositionComponent, TextComponent, VelocityComponent};
use crate::resource::SpriteManager;
use hecs::World;
use macroquad::prelude::*;
use macroquad_text::Fonts;

pub fn render_system(world: &World, sprite_manager: &SpriteManager, fonts: &Fonts)
{
    // 绘制 mushroom
    for (_, (pos, _)) in world
        .query::<(&PositionComponent, &VelocityComponent)>()
        .iter()
    {
        if let Some(texture) = sprite_manager.get_texture("mushroom")
        {
            // 使用浮点像素位置
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

    // 绘制文本
    for (_, (pos, _)) in world.query::<(&PositionComponent, &TextComponent)>().iter()
    {
        fonts.draw_text("你好，世界！", pos.x, pos.y, 30, WHITE);
    }
}
