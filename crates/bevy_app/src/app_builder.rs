use bevy_utils::{tracing::debug, HashSet};

use crate::{App, AppError, Plugin, PluginGroup};

pub struct AppBuilder {
    plugin_registry: Vec<Box<dyn Plugin>>,
    plugin_name_added: HashSet<String>,
    app: Option<App>,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            plugin_registry: Default::default(),
            plugin_name_added: Default::default(),
            app: Default::default(),
        }
    }
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_app(app: App) -> Self {
        Self {
            plugin_registry: Default::default(),
            plugin_name_added: Default::default(),
            app: Some(app),
        }
    }

    /// Adds a single [`Plugin`].
    ///
    /// One of Bevy's core principles is modularity. All Bevy engine features are implemented
    /// as [`Plugin`]s. This includes internal features like the renderer.
    ///
    /// Bevy also provides a few sets of default [`Plugin`]s. See [`add_plugins`](Self::add_plugins).
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// #
    /// # // Dummies created to avoid using `bevy_log`,
    /// # // which pulls in too many dependencies and breaks rust-analyzer
    /// # pub mod bevy_log {
    /// #     use bevy_app::prelude::*;
    /// #     #[derive(Default)]
    /// #     pub struct LogPlugin;
    /// #     impl Plugin for LogPlugin{
    /// #        fn build(&self, builder: &mut AppBuilder) {}
    /// #     }
    /// # }
    /// App::new().add_plugin(bevy_log::LogPlugin::default());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the plugin was already added to the application.
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        match self.add_boxed_plugin(Box::new(plugin)) {
            Ok(builder) => builder,
            Err(AppError::DuplicatePlugin { plugin_name }) => {
                panic!("Error adding plugin {plugin_name}: plugin was already added in application")
            }
        }
    }

    /// Boxed variant of `add_plugin`, can be used from a [`PluginGroup`]
    pub(crate) fn add_boxed_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
    ) -> Result<&mut Self, AppError> {
        debug!("added plugin: {}", plugin.name());
        if plugin.is_unique() && !self.plugin_name_added.insert(plugin.name().to_string()) {
            Err(AppError::DuplicatePlugin {
                plugin_name: plugin.name().to_string(),
            })?;
        }
        self.plugin_registry.push(plugin);
        Ok(self)
    }

    /// Checks if a [`Plugin`] has already been added.
    ///
    /// This can be used by plugins to check if a plugin they depend upon has already been
    /// added.
    pub fn is_plugin_added<T>(&self) -> bool
    where
        T: Plugin,
    {
        self.plugin_registry
            .iter()
            .any(|p| p.downcast_ref::<T>().is_some())
    }

    /// Returns a vector of references to any plugins of type `T` that have been added.
    ///
    /// This can be used to read the settings of any already added plugins.
    /// This vector will be length zero if no plugins of that type have been added.
    /// If multiple copies of the same plugin are added to the [`App`], they will be listed in insertion order in this vector.
    ///
    /// ```rust
    /// # use bevy_app::prelude::*;
    /// # #[derive(Default)]
    /// # struct ImagePlugin {
    /// #    default_sampler: bool,
    /// # }
    /// # impl Plugin for ImagePlugin {
    /// #    fn build(&self, builder: &mut AppBuilder) {}
    /// # }
    /// # let mut app = App::new();
    /// # app.add_plugin(ImagePlugin::default());
    /// let default_sampler = app.get_added_plugins::<ImagePlugin>()[0].default_sampler;
    /// ```
    pub fn get_added_plugins<T>(&self) -> Vec<&T>
    where
        T: Plugin,
    {
        self.plugin_registry
            .iter()
            .filter_map(|p| p.downcast_ref())
            .collect()
    }
    /// Adds a group of [`Plugin`]s.
    ///
    /// [`Plugin`]s can be grouped into a set by using a [`PluginGroup`].
    ///
    /// There are built-in [`PluginGroup`]s that provide core engine functionality.
    /// The [`PluginGroup`]s available by default are `DefaultPlugins` and `MinimalPlugins`.
    ///
    /// To customize the plugins in the group (reorder, disable a plugin, add a new plugin
    /// before / after another plugin), call [`build()`](PluginGroup::build) on the group,
    /// which will convert it to a [`PluginGroupBuilder`](crate::PluginGroupBuilder).
    ///
    /// ## Examples
    /// ```
    /// # use bevy_app::{prelude::*, PluginGroupBuilder, NoopPluginGroup as MinimalPlugins};
    /// #
    /// App::new()
    ///     .add_plugins(MinimalPlugins);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if one of the plugin in the group was already added to the application.
    pub fn add_plugins<T: PluginGroup>(&mut self, group: T) -> &mut Self {
        let builder = group.build();
        builder.finish(self)
    }

    pub fn build(mut self) -> App {
        // let mut app = self.app.unwrap_or_default();
        if self.app.is_none() {
            self.app = Some(App::default());
        }

        // loop {
        //     let registry = std::mem::replace(&mut self.plugin_registry, vec![]);
        //     for plugin in registry {
        //         // let _ = app.add_boxed_plugin(plugin);
        //         plugin.build(&mut self);
        //     }
        //     if self.plugin_registry.is_empty() {
        //         break;
        //     }
        // }
        self.build_recur(None);
        self.app.unwrap()
    }

    fn build_recur(&mut self, plugin: Option<Box<dyn Plugin>>) {
        if let Some(plugin) = plugin {
            plugin.build(self);
        }
        let registry = std::mem::replace(&mut self.plugin_registry, vec![]);
        for plugin in registry {
            self.build_recur(Some(plugin));
        }
    }

    pub fn app(&mut self) -> &mut App {
        if self.app.is_none() {
            self.app = Some(App::default());
        }

        self.app.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AppBuilder, Plugin};

    struct PluginA;
    impl Plugin for PluginA {
        fn build(&self, _builder: &mut AppBuilder) {}
    }
    struct PluginB;
    impl Plugin for PluginB {
        fn build(&self, _builder: &mut AppBuilder) {}
    }
    struct PluginC<T>(T);
    impl<T: Send + Sync + 'static> Plugin for PluginC<T> {
        fn build(&self, _builder: &mut AppBuilder) {}
    }
    struct PluginD;
    impl Plugin for PluginD {
        fn build(&self, _builder: &mut AppBuilder) {}
        fn is_unique(&self) -> bool {
            false
        }
    }

    #[test]
    fn can_add_two_plugins() {
        AppBuilder::new().add_plugin(PluginA).add_plugin(PluginB);
    }

    #[test]
    #[should_panic]
    fn cant_add_twice_the_same_plugin() {
        AppBuilder::new().add_plugin(PluginA).add_plugin(PluginA);
    }

    #[test]
    fn can_add_twice_the_same_plugin_with_different_type_param() {
        AppBuilder::new()
            .add_plugin(PluginC(0))
            .add_plugin(PluginC(true));
    }

    #[test]
    fn can_add_twice_the_same_plugin_not_unique() {
        AppBuilder::new().add_plugin(PluginD).add_plugin(PluginD);
    }

    // #[test]
    // #[should_panic]
    // fn cant_call_app_run_from_plugin_build() {
    //     struct PluginRun;
    //     impl Plugin for PluginRun {
    //         fn build(&self, builder: &mut AppBuilder) {
    //             app.run();
    //         }
    //     }
    //     AppBuilder::new().add_plugin(PluginRun);
    // }
}
