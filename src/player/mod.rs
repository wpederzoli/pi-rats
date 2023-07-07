use bevy::prelude::*;

pub mod cannon;
pub mod input;

use crate::{
    path_finding::a_star::{position_to_vec2, Position},
    platforms::cell::CELL_SIZE,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

use self::{
    cannon::{move_cannonball, shoot_cannon, ShootCannon},
    input::{input_system, InputSystem},
};
pub const PLAYER_LAYER: f32 = 2.;
pub const PLAYER_SPEED: f32 = 2.;

pub struct PlayerPlugin;

pub struct MovePlayer(pub Vec<Position>);

#[derive(Component)]
pub struct Player {
    pub cannon_ready: bool,
    pub finding_path: bool,
    steps: Vec<Position>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootCannon>()
            .add_event::<MovePlayer>()
            .add_startup_system(setup)
            .add_system(input_system)
            .add_system(shoot_cannon)
            .add_system(move_player)
            .add_system(move_cannonball);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(0, 255, 217, 255),
                custom_size: Some(Vec2::new(50., 70.)),
                ..default()
            },
            transform: Transform::from_xyz(
                (-WINDOW_WIDTH / 2.) + CELL_SIZE,
                WINDOW_HEIGHT / 2. - CELL_SIZE,
                PLAYER_LAYER,
            ),
            ..default()
        },
        Player {
            cannon_ready: true,
            finding_path: false,
            steps: Vec::new(),
        },
        InputSystem::default(),
    ));
}

fn move_player(
    mut move_event: EventReader<MovePlayer>,
    mut player: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = player.single_mut();

    for e in move_event.iter() {
        println!("move to this path: {:?}", e.0);
        player.steps = e.0.clone();
    }

    if player.steps.len() > 0 {
        let next_position = player.steps[0];
        let map_start = Vec2::new(
            (-WINDOW_WIDTH / 2.) + CELL_SIZE,
            WINDOW_HEIGHT / 2. - CELL_SIZE,
        );

        let pos_in_vec = position_to_vec2(&next_position, &map_start, CELL_SIZE);
        println!("pos vec: {:?}", pos_in_vec);

        if transform.translation.x < pos_in_vec.x {
            transform.translation.x += PLAYER_SPEED
        }
        if transform.translation.x > pos_in_vec.x {
            transform.translation.x -= PLAYER_SPEED
        }
        if transform.translation.y > pos_in_vec.y {
            transform.translation.y -= PLAYER_SPEED
        }
        if transform.translation.y < pos_in_vec.y {
            transform.translation.y += PLAYER_SPEED
        }

        if transform.translation.x == pos_in_vec.x && transform.translation.y == pos_in_vec.y {
            player.steps.remove(0);
        }
    }
}
