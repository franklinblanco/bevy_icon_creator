use bevy::{prelude::*, render::{camera::RenderTarget, view::RenderLayers}, utils::petgraph::matrix_graph::Zero};

use crate::{created_icons::CreatedIcons, image_ext::ImageExt, markers::{EntityGettingIconMarker, IconCreatorCameraMarker, IconCreatorEntityChildMarker, IconCreatorEntityParentMarker, IconCreatorSceneRootMarker, InIconCreatorSceneMarker, SceneOccupiedMarker}, needs_icon_marker::NeedsIconMarker, set_image_on_load::SetImageOnLoadMarker, utils::mark_all_children_with_component};

const MIN_FRAMES_TO_RENDER: u8 = 3;

/// In this update system the scenes will do the work of rendering to an image.
/// 
/// It should keep track of the amount of frames.
pub fn update_icon_creator_scenes(
    mut commands: Commands,
    mut scene_query: Query<(&InIconCreatorSceneMarker, &mut Visibility, &mut IconCreatorSceneRootMarker), With<SceneOccupiedMarker>>,
    mut scene_camera_query: Query<(&mut Camera, &InIconCreatorSceneMarker), With<IconCreatorCameraMarker>>,
    scene_entity_parent_query: Query<(Entity, &InIconCreatorSceneMarker), With<IconCreatorEntityParentMarker>>,
    scene_child_query: Query<(&Parent, &InIconCreatorSceneMarker, &EntityGettingIconMarker), With<IconCreatorEntityChildMarker>>,
    mut images: ResMut<Assets<Image>>,
    mut created_icons: ResMut<CreatedIcons>,
) {
    for (scene_marker, mut scene_root_visibility, mut scene_root_marker) in scene_query.iter_mut() {
        for (mut scene_camera, in_scene) in scene_camera_query.iter_mut() {
            if scene_marker.0 != in_scene.0 { continue; }
            for (scene_parent_entity, in_scene) in scene_entity_parent_query.iter() {
                if scene_marker.0 != in_scene.0 { continue; }
                for (scene_child_parent, in_scene, entity_getting_icon_marker) in scene_child_query.iter() {
                    if scene_child_parent.get() == scene_parent_entity && scene_marker.0 == in_scene.0  {
                        if scene_root_marker.0.is_zero() { // if it's the first frame
                            scene_camera.is_active = true;
                            *scene_root_visibility = Visibility::Visible;
                            // Create image
                            let camera_target_image_handle = if let Some(existing_image_handle) = created_icons.0.get(&entity_getting_icon_marker.id) {
                                existing_image_handle.clone()
                            } else {
                                let handle = images.add(Image::get_blank_image(512, 512));
                                created_icons.0.insert(entity_getting_icon_marker.id, handle.clone());
                                handle
                            };
                            scene_camera.target = RenderTarget::Image(camera_target_image_handle);
                        } else if scene_root_marker.0 >= MIN_FRAMES_TO_RENDER + entity_getting_icon_marker.extra_frames.unwrap_or(0) {
                            // Unoccupy
                            if let Some(mut entity_commands) = commands.get_entity(scene_parent_entity) {
                                entity_commands.remove::<SceneOccupiedMarker>();
                                scene_camera.is_active = false;
                                *scene_root_visibility = Visibility::Hidden;
                                scene_camera.target = RenderTarget::default();
                            }
                        }
                    }
                }
            }
        }
        scene_root_marker.0 += 1;
    }
}

#[allow(clippy::type_complexity)]
pub fn update_give_work_to_scenes(
    mut commands: Commands,
    needs_icon_query: Query<(Entity, &NeedsIconMarker)>,
    mut scene_query: Query<(Entity, &InIconCreatorSceneMarker, &RenderLayers), (With<IconCreatorSceneRootMarker>, Without<SceneOccupiedMarker>)>,
    scene_entity_parent_query: Query<(Entity, &InIconCreatorSceneMarker), With<IconCreatorEntityParentMarker>>,
) {
    for (needs_icon_entity, needs_icon_marker) in needs_icon_query.iter() {
        // move this entity to any unoccupied scene.
        // Then set the scene to occupied (Visibility::Visible & Camera.active = true & Occupied component)
        for (scene_entity, scene_marker, scene_render_layer) in scene_query.iter_mut() {
            for (scene_entity_parent, in_scene_marker) in scene_entity_parent_query.iter() {
                if scene_marker.0 == in_scene_marker.0 { // Is inside the scene root
                    if let Some(mut entity_commands) = commands.get_entity(needs_icon_entity) {
                        entity_commands
                            .set_parent(scene_entity_parent)
                            .insert(needs_icon_marker.transform.unwrap_or_default())
                            .insert(*scene_render_layer)
                            .insert(*in_scene_marker)
                            .insert(EntityGettingIconMarker {
                                extra_frames: needs_icon_marker.extra_frames,
                                id: needs_icon_marker.id,
                            })
                            .insert(IconCreatorEntityChildMarker)
                            .insert(VisibilityBundle::default())
                            .remove::<NeedsIconMarker>();
                        commands.entity(scene_entity).insert(SceneOccupiedMarker);
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_set_render_layers_recursively(
    mut commands: Commands,
    in_scene_but_no_render_layers_query: Query<(Entity, &RenderLayers, &InIconCreatorSceneMarker), (Added<RenderLayers>, Added<InIconCreatorSceneMarker>)>,
    children_query: Query<&Children>,
) {
    for (in_scene_entity, render_layers, in_icon_creator_scene_marker) in in_scene_but_no_render_layers_query.iter() {
        mark_all_children_with_component(&mut commands, in_scene_entity, &children_query, (*render_layers, VisibilityBundle::default(), *in_icon_creator_scene_marker), false);
    }
}

pub fn update_replace_images_on_ui_images(
    mut commands: Commands,
    mut marked_ui_images_query: Query<(Entity, &SetImageOnLoadMarker, &mut UiImage)>,
    created_icons: Res<CreatedIcons>,
) {
    for (entity, set_image_on_load_marker, mut ui_image) in marked_ui_images_query.iter_mut() {
        if let Some(image_handle) = created_icons.get_image_handle(&set_image_on_load_marker.0) {
            ui_image.texture = image_handle;
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<SetImageOnLoadMarker>();
            }
        }
    }
}