use bevy_asset::{self, Handle};
use bevy_render::{color::Color, renderer::{RenderResources}, shader::ShaderDefs, texture::Texture};
use bevy_math::Vec2;

/// A material with "standard" properties used in PBR lighting
#[derive(RenderResources, ShaderDefs)]
pub struct StandardMaterial {
    pub albedo: Color,
    /// Represented as roughness/metallic.
    pub pbr: Vec2,
    #[shader_def]
    pub albedo_texture: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub shaded: bool,
}

impl Default for StandardMaterial {
    fn default() -> Self {
        StandardMaterial {
            albedo: Color::rgb(1.0, 1.0, 1.0),
            pbr: Vec2::new(0.01, 0.08),
            albedo_texture: None,
            shaded: true,
        }
    }
}

impl From<Color> for StandardMaterial {
    fn from(color: Color) -> Self {
        StandardMaterial {
            albedo: color,
            ..Default::default()
        }
    }
}

impl From<Handle<Texture>> for StandardMaterial {
    fn from(texture: Handle<Texture>) -> Self {
        StandardMaterial {
            albedo_texture: Some(texture),
            ..Default::default()
        }
    }
}
