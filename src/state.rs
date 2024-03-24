use bevy::{ecs::{reflect::ReflectResource, system::Resource}, math::Vec3, reflect::Reflect};

#[derive(Resource, Debug, Reflect, Default, Clone)]
#[reflect(Resource)]
pub struct IconCreatorState {
    pub(crate) scenes: u8,
    pub(crate) world_pos: Vec3,
    pub(crate) render_layer: u8,
    pub(crate) light_intensity: f32,
}

impl IconCreatorState {
    pub fn new(scenes: u8, world_pos: Vec3, render_layer: u8, light_intensity: f32) -> Self {
        Self { scenes, world_pos, render_layer, light_intensity }
    }
}