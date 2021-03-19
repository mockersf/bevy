use super::{Diagnostic, DiagnosticId, Diagnostics};
use bevy_app::prelude::*;
use bevy_core::{Time, Timer};
use bevy_ecs::system::{IntoSystem, Res, ResMut};
use bevy_log::{debug, info};
use bevy_utils::{Duration, HashSet};

/// An App Plugin that logs diagnostics to the console
pub struct LogDiagnosticsPlugin {
    pub debug: bool,
    pub wait_duration: Duration,
}

/// State used by the [LogDiagnosticsPlugin]
struct LogDiagnosticsState {
    timer: Timer,
}

impl Default for LogDiagnosticsPlugin {
    fn default() -> Self {
        LogDiagnosticsPlugin {
            debug: false,
            wait_duration: Duration::from_secs(1),
        }
    }
}

/// The width which diagnostic names will be printed as
/// Plugin names should not be longer than this value
pub(crate) const MAX_LOG_NAME_WIDTH: usize = 32;

impl Plugin for LogDiagnosticsPlugin {
    fn build(&self, app: &mut bevy_app::AppBuilder) {
        app.insert_resource(LogDiagnosticsState {
            timer: Timer::new(self.wait_duration, true),
        });

        if self.debug {
            app.add_system_to_stage(
                CoreStage::PostUpdate,
                Self::log_diagnostics_debug_system.system(),
            );
        } else {
            app.add_system_to_stage(CoreStage::PostUpdate, Self::log_diagnostics_system.system());
        }
    }
}

impl LogDiagnosticsPlugin {
    fn log_diagnostic(diagnostic: &Diagnostic) {
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                info!(
                    target: "bevy diagnostic",
                    "{:<name_width$}: {:>12} (avg {:>})",
                    diagnostic.name,
                    // Suffix is only used for 's' as in seconds currently,
                    // so we reserve one column for it
                    format!("{:.6}{:1}", value, diagnostic.suffix),
                    // Do not reserve one column for the suffix in the average
                    // The ) hugging the value is more aesthetically pleasing
                    format!("{:.6}{:}", average, diagnostic.suffix),
                    name_width = MAX_LOG_NAME_WIDTH,
                );
            } else {
                info!(
                    target: "bevy diagnostic",
                    "{:<name_width$}: {:>}",
                    diagnostic.name,
                    format!("{:.6}{:}", value, diagnostic.suffix),
                    name_width = MAX_LOG_NAME_WIDTH,
                );
            }
        }
    }

    fn log_diagnostics_system(
        mut state: ResMut<LogDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
        log_diagnostics_config: Res<LogDiagnosticsConfig>,
    ) {
        if state.timer.tick(time.delta()).finished() {
            eprintln!("{:?}", *log_diagnostics_config);
            if !log_diagnostics_config.enabled {
                return;
            }
            match log_diagnostics_config.filter.as_ref() {
                Some(LogDiagnosticsFilter::Displayed(filter)) => {
                    for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                        Self::log_diagnostic(diagnostic);
                    }
                }
                Some(LogDiagnosticsFilter::Hidden(filter)) => {
                    for diagnostic in diagnostics
                        .iter()
                        .filter(|diagnostic| !filter.contains(&diagnostic.id))
                    {
                        Self::log_diagnostic(diagnostic);
                    }
                }
                None => {
                    for diagnostic in diagnostics.iter() {
                        Self::log_diagnostic(diagnostic);
                    }
                }
            }
        }
    }

    fn log_diagnostics_debug_system(
        mut state: ResMut<LogDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
        log_diagnostics_config: Res<LogDiagnosticsConfig>,
    ) {
        if state.timer.tick(time.delta()).finished() {
            if !log_diagnostics_config.enabled {
                return;
            }
            match log_diagnostics_config.filter.as_ref() {
                Some(LogDiagnosticsFilter::Displayed(filter)) => {
                    for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                        debug!("{:#?}\n", diagnostic);
                    }
                }
                Some(LogDiagnosticsFilter::Hidden(filter)) => {
                    for diagnostic in diagnostics
                        .iter()
                        .filter(|diagnostic| !filter.contains(&diagnostic.id))
                    {
                        debug!("{:#?}\n", diagnostic);
                    }
                }
                None => {
                    for diagnostic in diagnostics.iter() {
                        debug!("{:#?}\n", diagnostic);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LogDiagnosticsConfig {
    enabled: bool,
    filter: Option<LogDiagnosticsFilter>,
}

impl Default for LogDiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            filter: None,
        }
    }
}

#[derive(Clone, Debug)]
enum LogDiagnosticsFilter {
    Displayed(HashSet<DiagnosticId>),
    Hidden(HashSet<DiagnosticId>),
}

// methods in this impl are to use as a builder at setup
impl LogDiagnosticsConfig {
    /// Create a new `LogDiagnosticsConfig` that will display all diagnostics
    pub fn new() -> Self {
        Self {
            enabled: true,
            filter: None,
        }
    }

    ///
    pub fn disabled(&mut self) -> &mut Self {
        *self = Self {
            enabled: false,
            filter: self.filter.clone(),
        };
        self
    }

    pub fn displaying(&mut self, diagnostic_id: DiagnosticId) -> &mut Self {
        let filter = match self.filter.as_ref() {
            None => {
                let mut filter = HashSet::default();
                filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Displayed(filter))
            }
            Some(LogDiagnosticsFilter::Displayed(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Displayed(new_filter))
            }
            Some(LogDiagnosticsFilter::Hidden(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.remove(&diagnostic_id);
                Some(LogDiagnosticsFilter::Hidden(new_filter))
            }
        };
        *self = Self {
            enabled: self.enabled,
            filter,
        };
        self
    }

    pub fn hiding(&mut self, diagnostic_id: DiagnosticId) -> &mut Self {
        let filter = match self.filter.as_ref() {
            None => {
                let mut filter = HashSet::default();
                filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Hidden(filter))
            }
            Some(LogDiagnosticsFilter::Displayed(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.remove(&diagnostic_id);
                Some(LogDiagnosticsFilter::Displayed(new_filter))
            }
            Some(LogDiagnosticsFilter::Hidden(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Hidden(new_filter))
            }
        };
        *self = Self {
            enabled: self.enabled,
            filter,
        };
        self
    }
}

// methods to modify the config during run
impl LogDiagnosticsConfig {
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn display(&mut self, diagnostic_id: DiagnosticId) {
        let filter = match self.filter.as_ref() {
            None => None,
            Some(LogDiagnosticsFilter::Displayed(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Displayed(new_filter))
            }
            Some(LogDiagnosticsFilter::Hidden(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.remove(&diagnostic_id);
                Some(LogDiagnosticsFilter::Hidden(new_filter))
            }
        };
        *self = Self {
            enabled: self.enabled,
            filter,
        };
    }

    pub fn display_only(&mut self, diagnostic_id: DiagnosticId) {
        let mut filter = HashSet::default();
        filter.insert(diagnostic_id);

        *self = Self {
            enabled: self.enabled,
            filter: Some(LogDiagnosticsFilter::Displayed(filter)),
        };
    }

    pub fn hide(&mut self, diagnostic_id: DiagnosticId) {
        let filter = match self.filter.as_ref() {
            None => None,
            Some(LogDiagnosticsFilter::Displayed(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.remove(&diagnostic_id);
                Some(LogDiagnosticsFilter::Displayed(new_filter))
            }
            Some(LogDiagnosticsFilter::Hidden(filter)) => {
                let mut new_filter = filter.clone();
                new_filter.insert(diagnostic_id);
                Some(LogDiagnosticsFilter::Hidden(new_filter))
            }
        };
        *self = Self {
            enabled: self.enabled,
            filter,
        };
    }

    pub fn hide_only(&mut self, diagnostic_id: DiagnosticId) {
        let mut filter = HashSet::default();
        filter.insert(diagnostic_id);

        *self = Self {
            enabled: self.enabled,
            filter: Some(LogDiagnosticsFilter::Hidden(filter)),
        };
    }

    pub fn clear_filter(&mut self) {
        *self = Self {
            enabled: self.enabled,
            filter: None,
        };
    }
}
