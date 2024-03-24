use bevy::{ecs::{component::Component, reflect::ReflectComponent}, reflect::Reflect, utils::Uuid};

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct SetImageOnLoadMarker(pub Uuid);