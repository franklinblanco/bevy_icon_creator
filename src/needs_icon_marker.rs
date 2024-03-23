use bevy::{ecs::component::Component, reflect::Reflect, transform::components::Transform, utils::Uuid};

/// Put this marker on any entity. This crate will then find this marker and copy all the entities underneath this entity,
/// then it will spawn them in the icon creator scene and once N number of frames have passed then render it to a texture. 
#[derive(Debug, Component, Reflect, Default)]
pub struct NeedsIconMarker {
    /// The transform to be applied to the parent entity when it gets spawned in the icon creator scene.
    /// 
    /// If this is None then the transform will just be a Transform::default()
    pub(crate) transform: Option<Transform>,
    /// The amount of frames that will be waited for the final image to be rendered once spawned. 
    /// 
    /// This crate already waits 3 frames for the entity to be spawned.
    pub(crate) extra_frames: Option<u8>,
    /// With this identifier you will be able to look up the icon once it's created.
    pub(crate) id: Uuid,
}

impl NeedsIconMarker {
    pub fn new(id: Uuid) -> Self {
        Self { transform: None, extra_frames: None, id }
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }
    pub fn with_extra_frames(mut self, extra_frames: u8) -> Self {
        self.extra_frames = Some(extra_frames);
        self
    }
}