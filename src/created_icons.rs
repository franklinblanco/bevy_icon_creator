use bevy::{asset::Handle, ecs::{reflect::ReflectResource, system::Resource}, reflect::Reflect, render::texture::Image, utils::{HashMap, Uuid}};

#[derive(Resource, Default, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct CreatedIcons(pub(crate) HashMap<Uuid, Handle<Image>>);

impl CreatedIcons {
    pub fn get_image_handle(&self, uuid: &Uuid) -> Option<Handle<Image>> {
        self.0.get(uuid).cloned()
    }
}