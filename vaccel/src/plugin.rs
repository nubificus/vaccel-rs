use std::ffi::OsStr;
use std::sync::Arc;

use log::{debug, error};

use dashmap::DashMap;
use libloading::Library;

use vaccel_plugins::{
    InvocationError, PluginDeclaration, Result, VaccelPlugin, VaccelPluginFunctions,
};

/// A proxy object that makes sure a `VaccelPlugin` cannot outlive
/// the dynamic library it came from.
pub(crate) struct VaccelPluginProxy {
    name: String,
    plugin: Box<dyn vaccel_plugins::VaccelPlugin>,
    _lib: Arc<Library>,
}

impl VaccelPlugin for VaccelPluginProxy {
    fn supported(&self) -> &[VaccelPluginFunctions] {
        self.plugin.supported()
    }

    fn tf_session_load(&self, model_id: u64) -> Result<()> {
        debug!("In plugin proxy");
        self.plugin.tf_session_load(model_id)
    }

    fn tf_session_unload(&self, model_id: u64) -> Result<()> {
        self.plugin.tf_session_unload(model_id)
    }
}

#[derive(Default)]
pub(crate) struct Plugins {
    implementations: DashMap<VaccelPluginFunctions, Vec<Arc<VaccelPluginProxy>>>,
    libraries: Vec<Arc<Library>>,
}

impl VaccelPlugin for Plugins {
    fn supported(&self) -> &[VaccelPluginFunctions] {
        &[]
    }

    fn tf_session_load(&self, model_id: u64) -> Result<()> {
        debug!("Looking for plugin that implements tf_session_load");
        match self
            .implementations
            .get(&VaccelPluginFunctions::TFSessionLoad)
        {
            None => {
                error!("Could not find plugin");
                Err(InvocationError::NotImplemented)
            }
            Some(plugin) => {
                debug!("Calling implementation from {}", plugin[0].name);
                plugin[0].tf_session_load(model_id)
            }
        }
    }

    fn tf_session_unload(&self, model_id: u64) -> Result<()> {
        match self
            .implementations
            .get(&VaccelPluginFunctions::TFSessionUnload)
        {
            None => Err(InvocationError::NotImplemented),
            Some(plugin) => plugin[0].tf_session_unload(model_id),
        }
    }
}

impl Plugins {
    pub fn new() -> Self {
        Plugins::default()
    }

    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> crate::Result<()> {
        // Load the plugin library
        let lib = Arc::new(Library::new(library_path)?);

        let decl = lib
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();

        // Check that the plugin is using the same `rustc` and `core` version
        // as us.
        if decl.rustc_version != vaccel_plugins::RUSTC_VERSION
            || decl.core_version != vaccel_plugins::CORE_VERSION
        {
            return Err(crate::Error::Plugin("Version mismatch".to_string()));
        }

        let mut registrar = PluginRegistrar::new(lib.clone());
        (decl.register)(&mut registrar);

        // Parse the plugin to see what functions does it support
        // and link it in our functions DashMap
        let plugin = Arc::new(registrar.plugin.unwrap());
        debug!("Registered plugin: {}", plugin.name);

        for func in plugin.clone().supported() {
            debug!(
                "Registering function '{:?}' for plugin '{}'",
                func, plugin.name
            );
            match self.implementations.get_mut(func) {
                None => {
                    let functions = vec![plugin.clone()];
                    self.implementations.insert(*func, functions);
                }
                Some(mut functions) => functions.push(plugin.clone()),
            }
        }

        // and make sure keeps a reference to the library
        self.libraries.push(lib);

        Ok(())
    }
}

struct PluginRegistrar {
    plugin: Option<VaccelPluginProxy>,
    lib: Arc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Arc<Library>) -> Self {
        PluginRegistrar { plugin: None, lib }
    }
}

impl vaccel_plugins::PluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, name: &str, plugin: Box<dyn VaccelPlugin>) {
        self.plugin = Some(VaccelPluginProxy {
            name: name.to_string(),
            plugin,
            _lib: self.lib.clone(),
        })
    }
}
