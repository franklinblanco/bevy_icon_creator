use bevy::{app::{Plugin, Startup, Update}, math::Vec3};

use crate::{created_icons::CreatedIcons, register_types::RegisterTypesPlugin, setup::setup_icon_creation_scenes, state::IconCreatorState, update::{update_give_work_to_scenes, update_icon_creator_scenes, update_replace_images_on_ui_images, update_set_render_layers_recursively}};

const DEFAULT_SCENES_AMOUNT: u8 = 2;
const DEFAULT_WORLD_POSITION_FOR_ROOT: Vec3 = Vec3 { x: 0.0, y: -300.0, z: 0.0 };
const DEFAULT_RENDER_LAYER: u8 = 21;
const DEFAULT_LIGHT_INTENSITY: f32 = 14_000.0;

/// Either use `IconCreatorPlugin::default()` or `IconCreatorPlugin::with_config(scenes)`
pub struct IconCreatorPlugin {
    /// Value indicating the amount of scenes that this library will create to be able to generate textures.
    /// 
    /// A higher value will mean more performance impact but more capacity to render multiple textures at once.
    /// 
    /// Default is 2
    scenes: u8,
    /// The global coordinates of the Root of the scenes.
    /// 
    /// Default is Vec3 { x: 0.0, y: -300.0, z: 0.0 }
    world_pos: Vec3,
    /// The render layer everything inside this is going to be placed in 
    /// 
    /// Default is 21
    render_layer: u8,
    /// Default is 14,000
    light_intensity: f32,
}

impl Plugin for IconCreatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(RegisterTypesPlugin);
        app.insert_resource(IconCreatorState::new(self.scenes, self.world_pos, self.render_layer, self.light_intensity));
        app.insert_resource(CreatedIcons::default());

        app.add_systems(Startup, setup_icon_creation_scenes);
        app.add_systems(Update, (
            update_set_render_layers_recursively,
            update_give_work_to_scenes,
            update_icon_creator_scenes,
            update_replace_images_on_ui_images,
        ));
    }
}

impl Default for IconCreatorPlugin {
    fn default() -> Self {
        Self { scenes: DEFAULT_SCENES_AMOUNT, world_pos: DEFAULT_WORLD_POSITION_FOR_ROOT, render_layer: DEFAULT_RENDER_LAYER, light_intensity: DEFAULT_LIGHT_INTENSITY }
    }
}

impl IconCreatorPlugin {
    pub fn with_config(scenes: u8, world_pos: Vec3, render_layer: u8, light_intensity: f32) -> Self {
        Self {
            scenes,
            world_pos,
            render_layer,
            light_intensity,
        }
    }
    pub fn with_scenes(mut self, scenes: u8) -> Self {
        self.scenes = scenes;
        self
    }
    pub fn with_world_pos(mut self, world_pos: Vec3) -> Self {
        self.world_pos = world_pos;
        self
    }
    pub fn with_render_layer(mut self, render_layer: u8) -> Self {
        self.render_layer = render_layer;
        self
    }
    pub fn with_light_intensity(mut self, light_intensity: f32) -> Self {
        self.light_intensity = light_intensity;
        self
    }
}