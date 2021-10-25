use vaccel_plugins::export_plugin;
use vaccel_plugins::VaccelPluginFunctions;
use vaccel_plugins::{InvocationError, PluginRegistrar, Result, VaccelPlugin};

use env_logger::Env;
use log::{debug, error};

pub struct Noop;

const FUNCTIONS: &[VaccelPluginFunctions] = &[
    VaccelPluginFunctions::TFSessionLoad,
    VaccelPluginFunctions::TFSessionUnload,
];

impl VaccelPlugin for Noop {
    fn supported(&self) -> &[VaccelPluginFunctions] {
        FUNCTIONS
    }

    fn tf_session_load(&self, model_id: u64) -> Result<()> {
        match model_id {
            0 => {
                error!("[noop] Calling tf_session_load with invalid model id");
                Err(InvocationError::InvalidArgument(
                    "Unknown model id".to_owned(),
                ))
            }
            _ => {
                debug!("[noop] Loading TF session for model {}", model_id);
                Ok(())
            }
        }
    }

    fn tf_session_unload(&self, model_id: u64) -> Result<()> {
        match model_id {
            0 => {
                error!("[noop] Calling tf_session_unload with invalid model id");
                Err(InvocationError::InvalidArgument(
                    "Unknown model id".to_owned(),
                ))
            }
            _ => {
                debug!("[noop] Unloading TF session for model {}", model_id);
                Ok(())
            }
        }
    }
}

export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    registrar.register_plugin("vaccel-noop", Box::new(Noop));
}
