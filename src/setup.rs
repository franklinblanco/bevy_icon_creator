use bevy::{prelude::*, render::{camera::ScalingMode, view::RenderLayers}};

use crate::{markers::{IconCreatorEntityParentMarker, IconCreatorRootMarker, IconCreatorSceneRootMarker, InIconCreatorSceneMarker}, state::IconCreatorState};

pub fn setup_icon_creation_scenes(
    mut commands: Commands,
    icon_creator_state: Res<IconCreatorState>,
) {
    let icon_creator_root_entity = commands.spawn((
        IconCreatorRootMarker,
        TransformBundle::from_transform(Transform::from_translation(icon_creator_state.world_pos)),
        VisibilityBundle::default(),
        RenderLayers::layer(icon_creator_state.render_layer),
        Name::new("Icon Creator Root Marker")
    )).id();
    for scene_id in 0..icon_creator_state.scenes {
        // Scene Root
        let scene_root_entity = commands
        .spawn((
            IconCreatorSceneRootMarker(0),
            InIconCreatorSceneMarker(scene_id),
            TransformBundle::from_transform(Transform::from_translation(Vec3::X * 5.0 * scene_id as f32)),
            VisibilityBundle::default(),
            RenderLayers::layer(icon_creator_state.render_layer),
            Name::new(format!("Scene Root with id: {scene_id}")),
        ))
        .set_parent(icon_creator_root_entity)
        .id();
        // Camera 
        commands
            .spawn((
                InIconCreatorSceneMarker(scene_id),
                Camera3dBundle {
                    camera: Camera {
                        order: -1,
                        clear_color: ClearColorConfig::Custom(Color::NONE),
                        is_active: false,
                        hdr: true,
                        ..Default::default()
                    },
                    projection: Projection::Orthographic(OrthographicProjection {
                        scaling_mode: ScalingMode::Fixed { width: 1.0, height: 1.0 },
                        scale: 0.5,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
                RenderLayers::layer(icon_creator_state.render_layer),
                Name::new("Scene camera"),
                ))
            .set_parent(scene_root_entity);
        // Light
        commands
            .spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: icon_creator_state.light_intensity,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 2.5),
                    ..Default::default()
                },
                InIconCreatorSceneMarker(scene_id),
                RenderLayers::layer(icon_creator_state.render_layer),
                Name::new("Scene Point Light"),
            ))
            .set_parent(scene_root_entity);
        // Parent of the entities that will be spawned.
        commands
            .spawn((
                IconCreatorEntityParentMarker,
                InIconCreatorSceneMarker(scene_id),
                TransformBundle::default(),
                VisibilityBundle::default(),
                RenderLayers::layer(icon_creator_state.render_layer),
                Name::new("Scene Entity Parent"),
            ))
            .set_parent(scene_root_entity);
    }
}