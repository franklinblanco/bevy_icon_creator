use bevy::ecs::component::Component;

/// The root of all the scenes that will be generated from this plugin.
#[derive(Component, Debug)]
pub struct IconCreatorRootMarker;

#[derive(Component, Debug)]
pub struct IconCreatorSceneRootMarker;

#[derive(Component, Debug)]
pub struct IconCreatorCameraMarker;

#[derive(Component, Debug)]
pub struct IconCreatorLightMarker;

#[derive(Component, Debug)]
pub struct IconCreatorEntityParentMarker;

/// Everything inside a scene should contain this marker with the scene's id.
#[derive(Component, Debug)]
pub struct InIconCreatorSceneMarker(pub u8);