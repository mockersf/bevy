use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsConfig, LogDiagnosticsPlugin},
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Adds frame time diagnostics
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        .add_plugin(LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        // Uncomment this to add some render resource diagnostics:
        // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // Uncomment this to add an entity count diagnostics:
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add an asset count diagnostics:
        // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default())
        .add_system(toggle_diagnostics_system.system())
        .run();
}

fn toggle_diagnostics_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut log_diagnostics_config: ResMut<LogDiagnosticsConfig>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(KeyCode::Space) = event.key_code {
            if event.state == ElementState::Pressed {
                log_diagnostics_config.toggle();
            }
        }
    }
}
