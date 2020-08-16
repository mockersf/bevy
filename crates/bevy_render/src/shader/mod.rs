#[allow(clippy::module_inception)]
mod shader;
mod shader_defs;

#[cfg(not(target_arch = "wasm32"))]
mod shader_reflect;
#[cfg(feature = "naga-reflect")]
mod shader_reflect_naga;

pub use shader::*;
pub use shader_defs::*;

#[cfg(not(target_arch = "wasm32"))]
pub use shader_reflect::*;
pub struct ShaderLayout {
    pub bind_groups: Vec<BindGroupDescriptor>,
    pub vertex_buffer_layout: Vec<VertexBufferLayout>,
    pub entry_point: String,
}

pub const GL_VERTEX_INDEX: &str = "gl_VertexIndex";
pub const GL_INSTANCE_INDEX: &str = "gl_InstanceIndex";
