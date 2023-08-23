use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{input::InputSystemPlugin, platforms::PlatformsPlugin, players::PlayersPlugin};

pub mod input;
mod platforms;
mod players;

pub struct GamePlayPlugin;

impl PluginGroup for GamePlayPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputSystemPlugin)
            .add(PlayersPlugin)
            .add(PlatformsPlugin)
    }
}
