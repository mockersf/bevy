use bevy_asset::{self, Handle};
use bevy_reflect::TypeUuid;
use bevy_render::{color::Color, renderer::RenderResources, shader::ShaderDefs, texture::Texture};

/// A material with "standard" properties used in PBR lighting
/// Standard property values with pictures here https://google.github.io/filament/Material%20Properties.pdf
#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "dace545e-4bc6-4595-a79d-c224fc694975"]
pub struct StandardMaterial {
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything in between
    /// If used together with a base_color_texture, this is factored into the final base color
    /// as `base_color * base_color_texture_value`
    pub base_color: Color,
    #[shader_def]
    pub base_color_texture: Option<Handle<Texture>>,
    #[shader_def]
    pub metallic_roughness_texture: Option<Handle<Texture>>,
    #[shader_def]
    pub normal_map: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub double_sided: bool,
    #[shader_def]
    pub occlusion_texture: Option<Handle<Texture>>,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Color,
    #[shader_def]
    pub emissive_texture: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub unlit: bool,
    pub properties: Color,
}

impl Default for StandardMaterial {
    fn default() -> Self {
        StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            metallic_roughness_texture: None,
            normal_map: None,
            double_sided: false,
            occlusion_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            unlit: false,
            properties: Color::rgba(0.089, 0.01, 0.5, 0.0),
        }
    }
}

impl From<Color> for StandardMaterial {
    fn from(color: Color) -> Self {
        StandardMaterial {
            base_color: color,
            ..Default::default()
        }
    }
}

impl From<Handle<Texture>> for StandardMaterial {
    fn from(texture: Handle<Texture>) -> Self {
        StandardMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }
    }
}
