use bevy::prelude::*;

mod a_star;

pub struct PathFindingPlugin;

pub struct FindPath {
    pub start: Vec2,
    pub goal: Vec2,
}

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FindPath>().add_system(find_path);
    }
}

fn find_path(mut event: EventReader<FindPath>) {
    for e in event.iter() {
        println!("find a path: start: {:?}, goal: {:?}", e.start, e.goal);
    }
}
