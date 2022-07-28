use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

const FONT_SIZE: f32 = 3.0;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed mi quam, sagittis ac neque quis, mollis dignissim quam. Pellentesque finibus, enim a laoreet ullamcorper, ipsum massa egestas ipsum, sed dictum ipsum nunc sit amet elit. Sed interdum nisi at congue aliquet. Aenean non neque et metus malesuada sollicitudin. Morbi at mattis lectus. Pellentesque lectus urna, sollicitudin et mauris et, accumsan dignissim ex. Cras elit quam, sagittis ut felis non, faucibus sagittis eros. Morbi volutpat nibh sit amet dui cursus, in ultricies lectus finibus. Nunc at volutpat arcu.";

/// This example shows what happens when there is a lot of buttons on screen.
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .init_resource::<UiFont>()
        .add_startup_system(setup)
        .run();
}

struct UiFont(Handle<Font>);
impl FromWorld for UiFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        UiFont(asset_server.load("fonts/FiraSans-Bold.ttf"))
    }
}

fn setup(mut commands: Commands, font: Res<UiFont>, windows: Res<Windows>) {
    let using_2d = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<bool>().ok())
        .unwrap_or_default();
    let lines = std::env::args()
        .nth(2)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(230);
    let per_line = std::env::args()
        .nth(3)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(1100);

    info!("Displaying {} characters", lines * per_line);

    let mut text = LOREM_IPSUM.to_string();
    if per_line > LOREM_IPSUM.len() {
        text = text.repeat(per_line / LOREM_IPSUM.len() + 1);
    }
    text = text[0..per_line].to_string();

    commands.spawn_bundle(Camera2dBundle::default());

    if using_2d {
        info!("using bevy_text");
        let x = -windows.primary().width() / 2.0;
        let y = windows.primary().height() / 2.0;
        for i in 0..lines {
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    text.clone(),
                    TextStyle {
                        font: font.0.clone_weak(),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                transform: Transform::from_xyz(x, y - i as f32 * FONT_SIZE, 0.0),
                ..default()
            });
        }
    } else {
        info!("using bevy_ui");
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for _ in 0..lines {
                    parent.spawn_bundle(TextBundle::from_section(
                        text.clone(),
                        TextStyle {
                            font: font.0.clone_weak(),
                            font_size: FONT_SIZE,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }
            });
    }
}
