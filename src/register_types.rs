use bevy::app::Plugin;

use crate::{created_icons::CreatedIcons, markers::*, set_image_on_load::SetImageOnLoadMarker, state::IconCreatorState};

pub struct RegisterTypesPlugin;

impl Plugin for RegisterTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<CreatedIcons>()
            .register_type::<IconCreatorRootMarker>()
            .register_type::<IconCreatorSceneRootMarker>()
            .register_type::<IconCreatorCameraMarker>()
            .register_type::<IconCreatorLightMarker>()
            .register_type::<IconCreatorEntityParentMarker>()
            .register_type::<IconCreatorEntityChildMarker>()
            .register_type::<InIconCreatorSceneMarker>()
            .register_type::<EntityGettingIconMarker>()
            .register_type::<SceneOccupiedMarker>()
            .register_type::<SetImageOnLoadMarker>()
            .register_type::<IconCreatorState>()
        ;
    }
}