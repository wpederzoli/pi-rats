use bevy::{
    prelude::{Commands, Plugin, Query, Vec2},
    window::Window,
};

pub struct GamePlay;

mod background;
mod map;
mod platform;

impl Plugin for GamePlay {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    commands.spawn(background::BackgroundBundle::new(
        window.width(),
        window.height(),
    ));

    let map = map::Map::new(
        10,
        10,
        Vec2::new(window.width() / 10., window.height() / 10.),
    );

    map.spawn(
        &mut commands,
        &mut platform::PlatformBundle::new(map.cell_size.x * 3., map.cell_size.y * 5.),
        -4,
        2,
    );

    map.spawn(
        &mut commands,
        &mut platform::PlatformBundle::new(map.cell_size.x * 3., map.cell_size.y * 5.),
        1,
        2,
    );
}
