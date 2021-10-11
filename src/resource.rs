use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::tensorflow::models::{TensorflowModel, TensorflowSavedModel};

pub trait ResourceType<'a>: Serialize + Deserialize<'a> {
    /// Get id of the resource
    fn id(&self) -> u64;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Resource {
    /// A TensorFlow SavedModel
    TensorflowSavedModel(TensorflowSavedModel),
    /// A TensorFlow protobuf model
    TensorFlowModel(TensorflowModel),
}
