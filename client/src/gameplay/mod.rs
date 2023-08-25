use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{
    gameplay::GamePlay, input::InputSystemPlugin, platforms::PlatformsPlugin,
    players::PlayersPlugin,
};

mod gameplay;
pub mod input;
mod platforms;
mod players;

pub struct GamePlayPlugin;

impl PluginGroup for GamePlayPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GamePlay)
    }
}
