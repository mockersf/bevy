#[allow(clippy::module_inception)]
mod mesh;
/// Generation for some primitive shape meshes.
pub mod shape;

pub use mesh::*;

use crate::render_asset::RenderAssetPlugin;
use bevy_app::{App, AppBuilder, Plugin};
use bevy_asset::AddAsset;
use bevy_ecs::entity::Entity;

/// Adds the [`Mesh`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        let app = builder.app();
        app.add_asset::<Mesh>()
            .add_asset::<skinning::SkinnedMeshInverseBindposes>()
            .register_type::<skinning::SkinnedMesh>()
            .register_type::<Vec<Entity>>();
        builder.add_plugin(RenderAssetPlugin::<Mesh>::default());
    }
}
