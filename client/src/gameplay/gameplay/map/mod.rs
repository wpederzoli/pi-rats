use bevy::{
    prelude::{Bundle, Color, Commands, Component, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use self::map_component::MapComponent;

pub mod map_component;

#[derive(Component)]
pub struct Map {
    pub cell_size: Vec2,
    width: i16,
    height: i16,
}

#[derive(Bundle)]
pub struct MapBundle {
    #[bundle]
    graphics: SpriteBundle,
    map: Map,
}

impl MapBundle {
    pub fn new(width: i16, height: i16, cell_size: Vec2) -> MapBundle {
        MapBundle {
            graphics: SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(
                        width as f32 * cell_size.x,
                        height as f32 * cell_size.y,
                    )),
                    ..default()
                },
                ..default()
            },
            map: Map::new(width, height, cell_size),
        }
    }
}

impl Map {
    pub fn new(width: i16, height: i16, cell_size: Vec2) -> Self {
        Map {
            cell_size,
            width,
            height,
        }
    }

    pub fn spawn<T: MapComponent>(
        &self,
        commands: &mut Commands,
        component: &mut T,
        x: i16,
        y: i16,
    ) {
        let (x, y) = self.to_screen_coords(x, y);
        commands.spawn(component.init(x, y));
    }

    fn to_screen_coords(&self, x: i16, y: i16) -> (f32, f32) {
        let x = x as f32 * self.cell_size.x;
        let y = y as f32 * self.cell_size.y;

        (x, y)
    }
}
