use macroquad::prelude::*;
use miniquad::conf::Icon;
use std::path::Path;

pub fn window_conf() -> Conf
{
    let project_root = env!("CARGO_MANIFEST_DIR");
    let small_icon_path = Path::new(project_root).join("res/icon/SCG_small.raw");
    let medium_icon_path = Path::new(project_root).join("res/icon/SCG_medium.raw");
    let big_icon_path = Path::new(project_root).join("res/icon/SCG_big.raw");

    Conf {
        window_title: "Macroquad + Hecs + ResourceManager".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        high_dpi: true,
        sample_count: 1,
        icon: Some(Icon {
            small: std::fs::read(small_icon_path)
                .expect("Failed to read small icon")
                .try_into()
                .expect("Small icon size mismatch"),
            medium: std::fs::read(medium_icon_path)
                .expect("Failed to read medium icon")
                .try_into()
                .expect("Medium icon size mismatch"),
            big: std::fs::read(big_icon_path)
                .expect("Failed to read big icon")
                .try_into()
                .expect("Big icon size mismatch"),
        }),
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}
