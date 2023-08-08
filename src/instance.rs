use bytemuck::{Pod, Zeroable};
use cint::EncodedSrgb;
use mint::Vector2;

/// Defined the rotation and the scale of the sprite.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpriteTransform {
    /// How much the sprite should be stretched along it's own X and Y axes.
    pub scale: Vector2<f32>,
    /// Clockwise rotation of the sprite in the plane in degrees.
    pub rotation_deg: f32,
}

impl SpriteTransform {
    pub fn scaled(scale: impl Into<Vector2<f32>>) -> Self {
        Self {
            scale: scale.into(),
            ..Default::default()
        }
    }

    pub fn rotated_deg(rotation_deg: f32) -> Self {
        Self {
            rotation_deg,
            ..Default::default()
        }
    }

    pub fn rotated_rad(rotation_rad: f32) -> Self {
        Self {
            rotation_deg: rotation_rad.to_degrees(),
            ..Default::default()
        }
    }
}

impl Default for SpriteTransform {
    fn default() -> Self {
        Self {
            scale: [1.; 2].into(),
            rotation_deg: 0.,
        }
    }
}

/// Specification of how the sprite should appear on the screen.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpriteInstance {
    /// The world position of the sprite's center.
    pub position: Vector2<f32>,
    pub transform: SpriteTransform,
    /// RGB components of the sprite's colour.
    pub color: EncodedSrgb<u8>,
    /// The alpha channel of the drawn colour.
    /// Any values oustide of the `0..1` range will be clamped.
    /// All the sprites are considered non-premultiplied.
    pub opacity: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Zeroable, Pod)]
#[repr(C)]
pub(crate) struct RawSpriteInstance {
    pub(crate) position: [f32; 2],
    pub(crate) scale: [f32; 2],
    pub(crate) rotation_rad: f32,
    pub(crate) color: [f32; 4],
}

impl From<SpriteInstance> for RawSpriteInstance {
    fn from(val: SpriteInstance) -> Self {
        val.raw()
    }
}

impl SpriteInstance {
    pub(crate) fn raw(&self) -> RawSpriteInstance {
        let r = self.color.r;
        let g = self.color.g;
        let b = self.color.b;

        let [r, g, b]: [f32; 3] = [r, g, b].map(|component| (component as f32) / 255.);

        RawSpriteInstance {
            position: self.position.into(),
            rotation_rad: f32::to_radians(self.transform.rotation_deg),
            scale: self.transform.scale.into(),
            color: [r, g, b, self.opacity].map(|component| component.clamp(0., 1.)),
        }
    }

    pub(crate) fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawSpriteInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as _,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as _,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as _,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}