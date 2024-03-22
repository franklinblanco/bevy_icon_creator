use bevy::{app::Plugin, math::Vec3};

const DEFAULT_WORLD_POSITION_FOR_ROOT: Vec3 = Vec3 { x: 4000.0, y: -300.0, z: 4000.0 };

/// Either use `IconCreatorPlugin::default()` or `IconCreatorPlugin::with_config(scenes)`
pub struct IconCreatorPlugin {
    /// Value indicating the amount of scenes that this library will create to be able to generate textures.
    /// 
    /// A higher value will mean more performance impact but more capacity to render multiple textures at once.
    /// 
    /// Default is 1
    scenes: u8,
    /// The global coordinates of the Root of the scenes.
    /// 
    /// Default is Vec3 { x: 4000.0, y: -300.0, z: 4000.0 }
    world_pos: Vec3,
}


impl Default for IconCreatorPlugin {
    fn default() -> Self {
        Self { scenes: 1, world_pos: DEFAULT_WORLD_POSITION_FOR_ROOT }
    }
}

impl IconCreatorPlugin {
    pub fn with_config(scenes: u8, world_pos: Vec3) -> Self {
        Self {
            scenes,
            world_pos,
        }
    }
}

impl Plugin for IconCreatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}