use bevy::{
    core::FixedTimestep,
    gltf::*,
    math::{const_quat, const_vec3},
    prelude::*,
    scene::InstanceId,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(switch_scene),
        )
        .add_system(setup_scene_once_loaded)
        .add_system(gltf_animation_driver)
        .add_system(animate_light_direction)
        .run();
}

struct Example {
    model_name: &'static str,
    camera_transform: Transform,
    speed: f32,
}
impl Example {
    const fn new(model_name: &'static str, camera_transform: Transform, speed: f32) -> Self {
        Self {
            model_name,
            camera_transform,
            speed,
        }
    }
}

// const ANIMATIONS: [(&str, Transform, f32); 3] = [
const ANIMATIONS: [Example; 3] = [
    // https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0/AnimatedTriangle
    Example::new(
        "models/fox.glb",
        Transform {
            translation: const_vec3!([60.0, 100.0, 120.0]),
            rotation: const_quat!([0.0, 0.0, 0.0, 1.0]),
            scale: const_vec3!([1.0; 3]),
        },
        0.8,
    ),
    Example::new(
        "models/fox.glb",
        Transform {
            translation: const_vec3!([60.0, 100.0, 120.0]),
            rotation: const_quat!([0.0, 0.0, 0.0, 1.0]),
            scale: const_vec3!([1.0; 3]),
        },
        0.8,
    ),
    Example::new(
        "models/fox.glb",
        Transform {
            translation: const_vec3!([60.0, 100.0, 120.0]),
            rotation: const_quat!([0.0, 0.0, 0.0, 1.0]),
            scale: const_vec3!([1.0; 3]),
        },
        1.2,
    ),
];

struct CurrentScene {
    instance_id: InstanceId,
    animation: Handle<GltfAnimation>,
    speed: f32,
}

struct CurrentAnimation {
    start_time: f64,
    animation: GltfAnimation,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(CurrentScene {
        // Its instance id, to be able to check that it's loaded
        instance_id: scene_spawner
            .spawn(asset_server.load(&format!("{}#Scene0", ANIMATIONS[0].model_name))),
        // The handle to the first animation
        animation: asset_server.load(&format!("{}#Animation0", ANIMATIONS[0].model_name)),
        // The animation speed modifier
        speed: ANIMATIONS[0].speed,
    });

    // Add a camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: ANIMATIONS[0]
            .camera_transform
            .looking_at(Vec3::new(0.0, 40.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5000.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

// Switch the scene to the next one every 10 seconds
fn switch_scene(
    mut commands: Commands,
    scene_root: Query<
        Entity,
        (
            Without<Camera>,
            Without<DirectionalLight>,
            Without<PointLight>,
            Without<Parent>,
            Without<Handle<Mesh>>,
        ),
    >,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut current: Local<usize>,
    mut current_scene: ResMut<CurrentScene>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    *current = (*current + 1) % ANIMATIONS.len();

    // Despawn the existing scene, then start loading the next one
    commands.entity(scene_root.single()).despawn_recursive();
    current_scene.instance_id = scene_spawner
        .spawn(asset_server.load(&format!("{}#Scene0", ANIMATIONS[*current].model_name)));
    current_scene.animation = asset_server.load(&format!(
        "{}#Animation{}",
        ANIMATIONS[*current].model_name, *current
    ));
    current_scene.speed = ANIMATIONS[*current].speed;

    // Update the camera position
    *camera.single_mut() = ANIMATIONS[*current]
        .camera_transform
        .looking_at(Vec3::new(0.0, 40.0, 0.0), Vec3::Y);

    // Reset the current animation
    commands.remove_resource::<CurrentAnimation>();
}

// Setup the scene for animation once it is loaded, by adding the animation to a resource with
// the start time.
fn setup_scene_once_loaded(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    current_scene: Res<CurrentScene>,
    time: Res<Time>,
    mut done: Local<bool>,
    animations: Res<Assets<GltfAnimation>>,
) {
    // If the current scene resource has changed, start waiting for it to be loaded
    if current_scene.is_changed() {
        *done = false;
    }
    // Once the scene and the animation are loaded, start the animation
    if !*done && scene_spawner.instance_is_ready(current_scene.instance_id) {
        if let Some(animation) = animations.get(&current_scene.animation) {
            *done = true;
            commands.insert_resource(CurrentAnimation {
                start_time: time.seconds_since_startup(),
                animation: animation.clone(),
            })
        }
    }
}

// This animation driver is not made to work in the general case. It will work with only one
// animation per scene, and will ignore the specified interpolation method to only do linear.
fn gltf_animation_driver(
    mut animated: Query<(&mut Transform, &GltfAnimatedNode)>,
    current_animation: Option<ResMut<CurrentAnimation>>,
    current_scene: Res<CurrentScene>,
    time: Res<Time>,
) {
    if let Some(mut current_animation) = current_animation {
        let elapsed = (time.seconds_since_startup() - current_animation.start_time) as f32
            * current_scene.speed;
        let mut moved = false;
        for (mut transform, node) in animated.iter_mut() {
            let node_animations = current_animation
                .animation
                .node_animations
                .get(&node.index)
                .unwrap();
            for node_animation in node_animations.iter() {
                let mut keyframe_timestamps = node_animation.keyframe_timestamps.iter().enumerate();
                let mut step_start = keyframe_timestamps.next().unwrap();
                if elapsed < *step_start.1 {
                    continue;
                }
                for next in keyframe_timestamps {
                    if *next.1 > elapsed {
                        break;
                    }
                    step_start = next;
                }
                if step_start.0 == node_animation.keyframe_timestamps.len() - 1 {
                    continue;
                }
                moved = true;
                let step_end = node_animation.keyframe_timestamps[step_start.0 + 1];
                let lerp = (elapsed - *step_start.1) / (step_end - step_start.1);
                match &node_animation.keyframes {
                    GltfNodeAnimationKeyframes::Rotation(keyframes) => {
                        let rot_start = keyframes[step_start.0];
                        let mut rot_end = keyframes[step_start.0 + 1];
                        if rot_end.dot(rot_start) < 0.0 {
                            rot_end = -rot_end;
                        }
                        let result = Quat::from_array(rot_start.normalize().into())
                            .slerp(Quat::from_array(rot_end.normalize().into()), lerp);
                        transform.rotation = result;
                    }
                    GltfNodeAnimationKeyframes::Translation(keyframes) => {
                        let translation_start = keyframes[step_start.0];
                        let translation_end = keyframes[step_start.0 + 1];
                        let result = translation_start.lerp(translation_end, lerp);
                        transform.translation = result;
                    }
                    GltfNodeAnimationKeyframes::Scale(keyframes) => {
                        let scale_start = keyframes[step_start.0];
                        let scale_end = keyframes[step_start.0 + 1];
                        let result = scale_start.lerp(scale_end, lerp);
                        transform.scale = result;
                    }
                }
            }
        }
        if !moved {
            current_animation.start_time = time.seconds_since_startup();
        }
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}
