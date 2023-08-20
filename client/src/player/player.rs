use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
};

use crate::{platforms::cell::CELL_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{input::InputSystem, Player, PLAYER_LAYER};

pub struct SpawnPlayer(pub PlayerOne);
pub struct PlayerOne(pub bool);

#[derive(Component)]
pub struct PlayerControl;

//When a player is waiting
//and another player joins the room
//they send an event to spawn the second player
//
//When a player joins a room where another player is waiting
//they should stat in the right side of the screen
//and send an event to spawn the other player
//
//The spawnplayer function could receive a Left or Right param so we spawn
//the player at position Left or Right

pub fn spawn_player(commands: &mut Commands, player_one: PlayerOne) {
    if !player_one.0 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba_u8(0, 255, 217, 255),
                    custom_size: Some(Vec2::new(50., 70.)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (WINDOW_WIDTH / 2.) - CELL_SIZE,
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
        ));
    }
}
