use bevy::prelude::*;

pub mod cannon;
pub mod input;
pub mod player;

use crate::{
    path_finding::a_star::{helpers::position_to_vec2, position::Position},
    platforms::cell::CELL_SIZE,
    GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use self::{
    cannon::{shoot_cannon, ShootCannon},
    input::{input_system, InputSystem, SelectCell, Target},
    player::{PlayerBundle, PlayerControl},
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
            .add_event::<SelectCell>()
            .add_system(setup.in_schedule(OnEnter(GameState::Waiting)))
            .add_systems(
                (input_system, shoot_cannon, move_player).in_set(OnUpdate(GameState::GamePlay)),
            );
    }
}

pub const PLAYER_ONE_INITIAL_POS: Vec3 = Vec3::new(
    (-WINDOW_WIDTH / 2.) + CELL_SIZE,
    WINDOW_HEIGHT / 2. - CELL_SIZE,
    PLAYER_LAYER,
);
pub const PLAYER_TWO_INITIAL_POS: Vec3 = Vec3::new(
    WINDOW_WIDTH / 2. - CELL_SIZE,
    WINDOW_HEIGHT / 2. - CELL_SIZE,
    PLAYER_LAYER,
);

fn setup(mut commands: Commands) {
    let mut player = commands.spawn(PlayerBundle::new(PLAYER_ONE_INITIAL_POS));
    player.insert(InputSystem::default());
    player.insert(PlayerControl);
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgba_u8(0, 255, 217, 255),
    //             custom_size: Some(Vec2::new(50., 70.)),
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(
    //             (-WINDOW_WIDTH / 2.) + CELL_SIZE,
    //             WINDOW_HEIGHT / 2. - CELL_SIZE,
    //             PLAYER_LAYER,
    //         ),
    //         ..default()
    //     },
    //     Player {
    //         cannon_ready: true,
    //         finding_path: false,
    //         steps: Vec::new(),
    //     },
    //     InputSystem::default(),
    //     PlayerControl,
    // ));
}

fn move_player(
    mut move_event: EventReader<MovePlayer>,
    mut player: Query<(&mut Transform, &mut Player, &mut InputSystem)>,
) {
    let (mut transform, mut player, mut input) = player.single_mut();

    for e in move_event.iter() {
        player.steps = e.0.clone();
    }

    if player.steps.len() > 0 {
        let next_position = player.steps[0];
        let map_start = Vec2::new(
            (-WINDOW_WIDTH / 2.) + CELL_SIZE,
            WINDOW_HEIGHT / 2. - CELL_SIZE,
        );

        let pos_in_vec = position_to_vec2(&next_position, &map_start, CELL_SIZE);

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
    } else {
        if player.finding_path {
            input.destination = Target::none();
            player.finding_path = false;
        }
    }
}
