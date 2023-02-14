mod converter;
mod gilrs_system;

use bevy_app::{App, AppBuilder, CoreSet, Plugin, StartupSet};
use bevy_ecs::prelude::*;
use bevy_input::InputSystem;
use bevy_utils::tracing::error;
use gilrs::GilrsBuilder;
use gilrs_system::{gilrs_event_startup_system, gilrs_event_system};

#[derive(Default)]
pub struct GilrsPlugin;

impl Plugin for GilrsPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        match GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
        {
            Ok(gilrs) => {
                builder
                    .app()
                    .insert_non_send_resource(gilrs)
                    .add_startup_system(gilrs_event_startup_system.in_set(StartupSet::PreStartup))
                    .add_system(
                        gilrs_event_system
                            .before(InputSystem)
                            .in_base_set(CoreSet::PreUpdate),
                    );
            }
            Err(err) => error!("Failed to start Gilrs. {}", err),
        }
    }
}
