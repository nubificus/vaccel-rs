use thiserror::Error;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Error that can be returned by the invocation of a plugin function
#[derive(Debug, Error)]
pub enum InvocationError {
    /// An invalid argument was passed to a function
    #[error("Invalid argument")]
    InvalidArgument(String),

    /// A function is not implemented by the plugin
    #[error("Plugin does not support function")]
    NotImplemented,

    /// An implementation-specific error
    #[error("Underlying error")]
    Implementation { error_code: u64, msg: String },

    /// An unknown error occured. This should indicate an implementation
    /// bug
    #[error("BUG: Undefined error")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, InvocationError>;

/// The plugin API
///
/// This is the set of functions supported by the vAccel API.
/// A plugin needs to implement a sub-set of these functions,
/// that is to say that plugins are not required to implement *all*
/// of the functions. If a function is not implemented, it should
/// return `InvocationError::NotImplemented`. Default implementations
/// are provided to reflect that.
pub trait VaccelPlugin: Send + Sync {
    /// A function that returns a slice with the Functions supported by
    /// this plugin
    fn supported(&self) -> &[VaccelPluginFunctions];

    /// Load a TensorFlow model in memory creating a session
    fn tf_session_load(&self, _model_id: u64) -> Result<()> {
        Err(InvocationError::NotImplemented)
    }

    /// Unload TensorFlow session
    fn tf_session_unload(&self, _model_id: u64) -> Result<()> {
        Err(InvocationError::NotImplemented)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum VaccelPluginFunctions {
    TFSessionLoad,
    TFSessionUnload,
    TFSessionRun,
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, name: &str, function: Box<dyn VaccelPlugin>);
}

/// A descriptor for a plugin implementation
pub struct PluginDeclaration {
    /// Version of `rustc` used to build the plugin
    pub rustc_version: &'static str,

    /// Version of the `core` crate used to build the plugin
    pub core_version: &'static str,

    /// A call-back function that registers the plugin with vAccel
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

/// A macro that facilitates defining a plugin descriptor
/// from plugin implementations
#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}
