use std::f32::consts::PI;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::{
        DirectionalLightShadowMap, NotShadowCaster, NotShadowReceiver, PointLightRange,
        PointLightShadowMap,
    },
    prelude::*,
    scene::InstanceId,
    transform::hierarchy::BuildChildren,
};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 50.0f32,
        })
        .insert_resource(PointLightShadowMap {
            size: 2_usize.pow(13),
        })
        .insert_resource(DirectionalLightShadowMap {
            size: 2_usize.pow(13),
        })
        .insert_resource(PointLightRange {
            minimum_illuminance: 0.20,
        })
        .insert_resource(Scenes {
            interior: None,
            exterior: None,
        })
        .insert_resource(WindowDescriptor {
            decorations: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(animate_camera)
        .add_system(scene_update)
        .add_system(night_and_day)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    let exterior = scene_spawner.spawn(asset_server.load("models/exterior/exterior.gltf#Scene0"));
    let interior = scene_spawner.spawn(asset_server.load("models/interior/interior.gltf#Scene0"));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-16., 6., 1.0).looking_at(Vec3::new(0.0, 1., 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.insert_resource(Scenes {
        interior: Some(interior),
        exterior: Some(exterior),
    });

    commands
        .spawn_bundle(DirectionalLightBundle {
            // transform: Transform::from_xyz(-16.0, 6.0, 1.0),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Sun);
}

struct Scenes {
    interior: Option<InstanceId>,
    exterior: Option<InstanceId>,
}

#[derive(Component)]
struct Interior;
#[derive(Component)]
struct Exterior;
#[derive(Component)]
struct Sun;

fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    mut scene_instance: ResMut<Scenes>,
    mut lights: Query<(&mut PointLight, &Name)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(instance_id) = scene_instance.interior {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
            entity_iter.for_each(|entity| {
                commands.entity(entity).insert(Interior);
                if let Ok((mut light, name)) = lights.get_mut(entity) {
                    info!("found {:?} at intensity {:?}", name, light.intensity);
                    info!("will cast shadows!");
                    light.shadows_enabled = true;
                }
            });
            scene_instance.interior = None;
        }
    }
    if let Some(instance_id) = scene_instance.exterior {
        let sphere = meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.05,
            ..Default::default()
        }));
        let material = materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..Default::default()
        });
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
            entity_iter.for_each(|entity| {
                commands.entity(entity).insert(Exterior);
                if let Ok((mut light, name)) = lights.get_mut(entity) {
                    info!("found {:?} at intensity {:?}", name, light.intensity);
                    if name.contains("streetlight") {
                        info!("will cast shadows!");
                        light.shadows_enabled = true;
                    }
                    if name.contains("smallbulb") && !name.contains("-no") {
                        info!("will cast shadows!");
                        light.shadows_enabled = true;
                    }
                    if name.contains("smallbulb") {
                        commands
                            .entity(entity)
                            .with_children(|p| {
                                p.spawn_bundle(PbrBundle {
                                    mesh: sphere.clone(),
                                    material: material.clone(),
                                    ..Default::default()
                                });
                            })
                            .insert_bundle((NotShadowCaster, NotShadowReceiver));
                    }
                }
            });
            scene_instance.exterior = None;
        }
    }
}

fn night_and_day(
    input: Res<Input<KeyCode>>,
    mut ambient: ResMut<AmbientLight>,
    mut dl: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut pli: Query<(&GlobalTransform, &mut PointLight, &Name), (With<Interior>, Without<Exterior>)>,
    mut ple: Query<(&GlobalTransform, &mut PointLight, &Name), (With<Exterior>, Without<Interior>)>,
    mut time_stopped: Local<bool>,
    mut light_state: Local<LightState>,
) {
    for (i, k) in [
        KeyCode::A,
        KeyCode::B,
        KeyCode::C,
        KeyCode::D,
        KeyCode::E,
        KeyCode::F,
        KeyCode::G,
        KeyCode::H,
        KeyCode::I,
        KeyCode::J,
        KeyCode::K,
        KeyCode::L,
    ]
    .iter()
    .enumerate()
    {
        if input.just_pressed(*k) {
            info!("pressed {:?}", k);
            let nb_interior = pli.iter().count();
            if let Some((_, mut light, name)) = pli.iter_mut().chain(ple.iter_mut()).nth(i) {
                info!("changing a light {:?}", name);
                if light.intensity > 0.0 {
                    light.intensity = 0.0;
                } else {
                    if i < nb_interior {
                        light.intensity = 1200.0;
                    } else {
                        light.intensity = 1000.0;
                    }
                }
            }
        }
    }
    if input.just_pressed(KeyCode::Space) {
        *time_stopped = !*time_stopped;
        info!("time: {:?}", *time_stopped);
    }
    let (t, mut dl) = dl.single_mut();
    let (x, _, _) = t.rotation.to_euler(EulerRot::XYZ);
    dl.illuminance = (-x).max(0.0) * 142000.0;
    ambient.brightness = (dl.illuminance / 400000.0).max(0.01);

    let current_state = if x > 0.15 {
        LightState::Night
    } else if x > -0.25 {
        LightState::Twilight
    } else {
        LightState::Day
    };
    if current_state != *light_state {
        if !*time_stopped {
            match current_state {
                LightState::Day => {
                    for (_, mut light, _) in pli.iter_mut() {
                        light.intensity = 0.0;
                    }
                    for (_, mut light, _) in ple.iter_mut() {
                        light.intensity = 0.0;
                    }
                }
                LightState::Twilight => {
                    for (_, mut light, _) in pli.iter_mut() {
                        light.intensity = 0.0;
                    }
                    for (_, mut light, name) in ple.iter_mut() {
                        if name.contains("street") {
                            light.intensity = 1000.0;
                        } else if name.contains("smallbulb") {
                            light.intensity = 0.0;
                        }
                    }
                }
                LightState::Night => {
                    for (_, mut light, _) in pli.iter_mut() {
                        light.intensity = 1200.0;
                    }
                    for (_, mut light, name) in ple.iter_mut() {
                        if name.contains("street") {
                            light.intensity = 1000.0;
                        } else if name.contains("smallbulb") {
                            light.intensity = 200.0;
                        }
                    }
                }
            }
            *light_state = current_state;
        }
        info!("{:?}", *light_state);
    }
}

#[derive(PartialEq, Debug)]
enum LightState {
    Day,
    Twilight,
    Night,
}
impl Default for LightState {
    fn default() -> Self {
        LightState::Day
    }
}

fn animate_camera(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        (
            With<Camera>,
            Without<Sun>,
            Without<DirectionalLight>,
            Without<PointLight>,
        ),
    >,
    mut sun: Query<&mut Transform, With<Sun>>,
) {
    let door_name = Name::new("dOORS_2");
    for (transform, name) in door.iter() {
        if *name == door_name {
            for mut camera_transform in query.iter_mut() {
                *camera_transform = Transform {
                    translation: transform.translation
                        + Quat::from_rotation_y(115.0 / 360.0 * PI * 2.0)
                            .mul_vec3(Vec3::new(0.0, 0.0, -14.0))
                        + Vec3::new(0.0, 3.5, 0.0),
                    ..Default::default()
                }
                .looking_at(transform.translation + Vec3::new(0.0, 3.0, 0.0), Vec3::Y);
            }
        }
    }
    // for mut transform in query.iter_mut() {
    //     *transform = Transform::from_xyz(
    //         -16. + (time.seconds_since_startup() / 10.0).sin() as f32 * 4.5,
    //         3.,
    //         1.0 + (time.seconds_since_startup() / 10.0).cos() as f32 * 6.5,
    //     )
    //     .looking_at(Vec3::new(0.0, 1., 0.0), Vec3::Y);
    // }

    for mut transform in sun.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 20.0,
            0.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}
