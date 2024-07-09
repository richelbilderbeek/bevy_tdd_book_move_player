use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let velocity = Vec2::new(0.2, 0.1);
    let mut app = create_app(velocity);
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);
    app.add_plugins(DefaultPlugins);
    app.run();
}
