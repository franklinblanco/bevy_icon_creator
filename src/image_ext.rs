use bevy::{prelude::default, render::{render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, texture::Image}};

pub trait ImageExt {
    fn get_blank_image(width: u32, height: u32) -> Self;
}

impl ImageExt for Image {
    fn get_blank_image(width: u32, height: u32) -> Self {
        let size = Extent3d {
            width,
            height,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        image.resize(size);
        image
    }
}