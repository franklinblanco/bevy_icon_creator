use bevy::{ecs::{component::Component, reflect::ReflectComponent}, reflect::Reflect, utils::Uuid};

/// The root of all the scenes that will be generated from this plugin.
#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorRootMarker;

/// u8 is for frames_elapsed
#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorSceneRootMarker(pub(crate) u8);

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorCameraMarker;

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorLightMarker;

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorEntityParentMarker;

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct IconCreatorEntityChildMarker;

/// Everything inside a scene should contain this marker with the scene's id.
#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct InIconCreatorSceneMarker(pub(crate) u8);

#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct EntityGettingIconMarker {
    pub(crate) extra_frames: Option<u8>,
    pub(crate) id: Uuid,
}

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct SceneOccupiedMarker;