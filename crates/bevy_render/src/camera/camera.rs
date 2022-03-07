use crate::{
    camera::CameraProjection, prelude::Image, render_asset::RenderAssets,
    render_resource::TextureView, view::ExtractedWindows,
};
use bevy_asset::{AssetEvent, Assets, Handle};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::{DetectChanges, QueryState},
    query::Added,
    reflect::ReflectComponent,
    system::{QuerySet, Res},
};
use bevy_math::{Mat4, UVec2, Vec2, Vec3};
use bevy_reflect::{Reflect, ReflectDeserialize};
use bevy_transform::components::GlobalTransform;
use bevy_utils::HashSet;
use bevy_window::{Window, WindowCreated, WindowId, WindowResized, Windows};
use serde::{Deserialize, Serialize};
use wgpu::Extent3d;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Camera {
    pub projection_matrix: Mat4,
    pub name: Option<String>,
    #[reflect(ignore)]
    pub target: RenderTarget,
    #[reflect(ignore)]
    pub depth_calculation: DepthCalculation,
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub enum RenderTarget {
    /// Window to which the camera's view is rendered.
    Window(WindowId),
    /// Image to which the camera's view is rendered.
    Image(Handle<Image>),
}

impl Default for RenderTarget {
    fn default() -> Self {
        Self::Window(Default::default())
    }
}

impl RenderTarget {
    pub fn get_window(&self) -> Option<WindowId> {
        if let RenderTarget::Window(id) = self {
            Some(id.to_owned())
        } else {
            None
        }
    }
    pub fn get_image(&self) -> Option<Handle<Image>> {
        if let RenderTarget::Image(handle) = self {
            Some(handle.to_owned())
        } else {
            None
        }
    }
    pub fn get_texture_view<'a>(
        &self,
        windows: &'a ExtractedWindows,
        images: &'a RenderAssets<Image>,
    ) -> Option<&'a TextureView> {
        match self {
            RenderTarget::Window(window_id) => windows
                .get(window_id)
                .and_then(|window| window.swap_chain_texture.as_ref()),
            RenderTarget::Image(image_handle) => {
                images.get(image_handle).map(|image| &image.texture_view)
            }
        }
    }
    /// Attempts to convert a `RenderTarget` into a `SizedTarget` trait object.
    pub fn as_sized_target<'a>(
        &self,
        windows: &'a Windows,
        images: &'a Assets<Image>,
    ) -> Option<&'a dyn SizedTarget> {
        Some(match self {
            RenderTarget::Window(window_id) => windows.get(*window_id)? as &'a dyn SizedTarget,
            RenderTarget::Image(image_handle) => images.get(image_handle)? as &'a dyn SizedTarget,
        })
    }
    pub fn get_physical_size(&self, windows: &Windows, images: &Assets<Image>) -> Option<UVec2> {
        self.as_sized_target(windows, images)?.get_physical_size()
    }
    pub fn get_logical_size(&self, windows: &Windows, images: &Assets<Image>) -> Option<Vec2> {
        self.as_sized_target(windows, images)?.get_logical_size()
    }
    // Check if this render target is contained in the given changed windows or images.
    fn is_changed(
        &self,
        changed_window_ids: &[WindowId],
        changed_image_handles: &HashSet<&Handle<Image>>,
    ) -> bool {
        match self {
            RenderTarget::Window(window_id) => changed_window_ids.contains(window_id),
            RenderTarget::Image(image_handle) => changed_image_handles.contains(&image_handle),
        }
    }
}

/// A trait used to find the size of a `RenderTarget`.
pub trait SizedTarget {
    fn get_physical_size(&self) -> Option<UVec2>;
    fn get_logical_size(&self) -> Option<Vec2>;
}
impl SizedTarget for Image {
    fn get_physical_size(&self) -> Option<UVec2> {
        let Extent3d { width, height, .. } = self.texture_descriptor.size;
        Some(UVec2::new(width, height))
    }
    fn get_logical_size(&self) -> Option<Vec2> {
        let Extent3d { width, height, .. } = self.texture_descriptor.size;
        Some(Vec2::new(width as f32, height as f32))
    }
}
impl SizedTarget for Window {
    fn get_physical_size(&self) -> Option<UVec2> {
        Some(UVec2::new(self.physical_width(), self.physical_height()))
    }
    fn get_logical_size(&self) -> Option<Vec2> {
        Some(Vec2::new(self.width(), self.height()))
    }
}
// Makes it possible to use a trait object in a generic function with a `SizedTarget` trait bound.
impl SizedTarget for &dyn SizedTarget {
    fn get_physical_size(&self) -> Option<UVec2> {
        (*self).get_physical_size()
    }
    fn get_logical_size(&self) -> Option<Vec2> {
        (*self).get_logical_size()
    }
}

#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
#[reflect_value(Serialize, Deserialize)]
pub enum DepthCalculation {
    /// Pythagorean distance; works everywhere, more expensive to compute.
    Distance,
    /// Optimization for 2D; assuming the camera points towards -Z.
    ZDifference,
}

impl Default for DepthCalculation {
    fn default() -> Self {
        DepthCalculation::Distance
    }
}

impl Camera {
    /// Given a position in world space, use the camera's position and a render target to compute
    /// the screen space coordinates.
    ///
    /// ## Examples
    ///
    /// If you have the camera's target already available, an [`Image`] or [`Window`], you can pass
    /// them into the function without needing to use an unnecessary resource.:
    ///
    /// ```no_run
    /// # use bevy_window::Windows;
    /// # use bevy_math::Vec3;
    /// # use bevy_ecs::prelude::Res;
    /// # use bevy_render::prelude::{PerspectiveCameraBundle};
    /// # use bevy_asset::Handle;
    /// fn my_system(windows: Res<Windows>) {
    ///     // ...
    ///     # let PerspectiveCameraBundle{ camera, ref global_transform, ..} = PerspectiveCameraBundle::new_3d();
    ///     let window = windows.get(camera.target.get_window().unwrap()).unwrap();
    ///     # let world_pos = Vec3::default();
    ///     camera.world_to_screen(window, global_transform, world_pos).unwrap();
    /// }
    /// ```
    ///
    /// If you only have the [`Camera`] or the [`RenderTarget`], you can instead use
    /// [`RenderTarget::as_sized_target`]:
    ///
    /// ```no_run
    /// # use bevy_window::Windows;
    /// # use bevy_math::Vec3;
    /// # use bevy_ecs::prelude::Res;
    /// # use bevy_asset::Assets;
    /// # use bevy_render::prelude::{Image, PerspectiveCameraBundle};
    /// fn my_system(windows: Res<Windows>, images: Assets<Image>) {
    ///     # let PerspectiveCameraBundle{ camera, ref global_transform, ..} = PerspectiveCameraBundle::new_3d();
    ///     # let world_pos = Vec3::default();
    ///     // ...
    ///     let sized_target = camera.target.as_sized_target(&*windows, &images).unwrap();
    ///     camera.world_to_screen(&sized_target, global_transform, world_pos).unwrap();
    /// }
    /// ```
    ///
    /// Note that the second example uses dynamic dispatch via a trait object, whereas the first
    /// method uses static dispatch and may be faster. In addition, the first example only requires
    ///
    /// To get the coordinates in Normalized Device Coordinates, you should use
    /// [`world_to_ndc`](Self::world_to_ndc).
    pub fn world_to_screen(
        &self,
        target: &impl SizedTarget,
        camera_transform: &GlobalTransform,
        world_position: Vec3,
    ) -> Option<Vec2> {
        let window_size = target.get_logical_size()?;
        // NDC z-values outside of 0 < z < 1 are outside the camera's near and far planes. We don't
        // check x or y because it's useful to report coordinates of objects as they leave the edges
        // of the screen.
        let ndc_space_coords = self
            .world_to_ndc(camera_transform, world_position)
            .filter(|pos| pos.z >= 0.0 && pos.z <= 1.0)?;
        // Once in NDC space, we can discard the z element and rescale x/y to fit the screen
        Some((ndc_space_coords.truncate() + 1.0) / 2.0 * window_size)
    }

    /// Given a position in world space, use the camera to compute the Normalized Device Coordinates.
    ///
    /// Values returned will be between -1.0 and 1.0 when the position is in screen space.
    /// To get the coordinates in the render target dimensions, you should use
    /// [`world_to_screen`](Self::world_to_screen).
    pub fn world_to_ndc(
        &self,
        camera_transform: &GlobalTransform,
        world_position: Vec3,
    ) -> Option<Vec3> {
        // Build a transform to convert from world to NDC using camera data
        let world_to_ndc: Mat4 =
            self.projection_matrix * camera_transform.compute_matrix().inverse();
        let unchecked_ndc = world_to_ndc.project_point3(world_position);
        Some(unchecked_ndc).filter(|x| !x.is_nan())
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_system<T: CameraProjection + Component>(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    mut queries: QuerySet<(
        QueryState<(Entity, &mut Camera, &mut T)>,
        QueryState<Entity, Added<Camera>>,
    )>,
) {
    let mut changed_window_ids = Vec::new();
    // handle resize events. latest events are handled first because we only want to resize each
    // window once
    for event in window_resized_events.iter().rev() {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    // handle resize events. latest events are handled first because we only want to resize each
    // window once
    for event in window_created_events.iter().rev() {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    let changed_image_handles: HashSet<&Handle<Image>> = image_asset_events
        .iter()
        .filter_map(|event| {
            if let AssetEvent::Modified { handle } = event {
                Some(handle)
            } else {
                None
            }
        })
        .collect();

    let mut added_cameras = vec![];
    for entity in &mut queries.q1().iter() {
        added_cameras.push(entity);
    }
    for (entity, mut camera, mut camera_projection) in queries.q0().iter_mut() {
        if camera
            .target
            .is_changed(&changed_window_ids, &changed_image_handles)
            || added_cameras.contains(&entity)
            || camera_projection.is_changed()
        {
            if let Some(size) = camera.target.get_logical_size(&windows, &images) {
                camera_projection.update(size.x, size.y);
                camera.projection_matrix = camera_projection.get_projection_matrix();
                camera.depth_calculation = camera_projection.depth_calculation();
            }
        }
    }
}
