use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};

pub fn app_default(title: String) -> AppBuilder {
    let window_config: WindowDescriptor = WindowDescriptor {
        title,
        width: 800,
        height: 600,
        vsync: true,
        resizable: false,
        mode: WindowMode::Windowed,
        ..Default::default()
    };

    let anti_alias_config: Msaa = Msaa { samples: 4 };
    let clear_background: ClearColor = ClearColor(Color::rgb(0.02, 0.03, 0.03));

    let mut app_builder = App::build();
    app_builder
        .add_resource(anti_alias_config)
        .add_resource(window_config)
        .add_resource(clear_background)
        .add_default_plugins();
    app_builder
}

