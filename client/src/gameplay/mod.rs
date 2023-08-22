use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::players::PlayersPlugin;

mod players;

pub struct GamePlayPlugin;

impl PluginGroup for GamePlayPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayersPlugin)
    }
}
