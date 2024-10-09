use macroquad::prelude::*;
use miniquad::conf::Icon;

pub fn window_conf() -> Conf
{
    Conf {
        window_title: "Macroquad + Hecs + ResourceManager".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        high_dpi: true,
        sample_count: 1,
        icon: Some(Icon {
            small: include_bytes!("../res/icon/SCG_small.raw")
                .to_vec()
                .try_into()
                .expect("Small icon size mismatch"),
            medium: include_bytes!("../res/icon/SCG_medium.raw")
                .to_vec()
                .try_into()
                .expect("Medium icon size mismatch"),
            big: include_bytes!("../res/icon/SCG_big.raw")
                .to_vec()
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
