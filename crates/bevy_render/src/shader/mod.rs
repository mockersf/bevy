#[allow(clippy::module_inception)]
mod preprocessor;
mod shader;
mod shader_defs;

#[cfg(not(target_arch = "wasm32"))]
mod shader_reflect;

pub use shader::*;
pub use shader_defs::*;

#[cfg(not(target_arch = "wasm32"))]
pub use shader_reflect::*;
pub struct ShaderLayout {
    pub bind_groups: Vec<crate::pipeline::BindGroupDescriptor>,
    pub vertex_buffer_layout: Vec<crate::pipeline::VertexBufferLayout>,
    pub entry_point: String,
}

pub const GL_VERTEX_INDEX: &str = "gl_VertexIndex";
pub const GL_INSTANCE_INDEX: &str = "gl_InstanceIndex";
