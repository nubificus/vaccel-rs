use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::resource::ResourceType;
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TensorflowSavedModel {
    id: u64,
    export_dir: Option<PathBuf>,
    model: Option<Vec<u8>>,
    checkpoint: Option<Vec<u8>>,
    var_index: Option<Vec<u8>>,
}

#[derive(Default)]
pub struct TensorflowSavedModelBuilder {
    export_dir: Option<PathBuf>,
    model: Option<Vec<u8>>,
    checkpoint: Option<Vec<u8>>,
    var_index: Option<Vec<u8>>,
}

impl TensorflowSavedModelBuilder {
    fn new() -> Self {
        TensorflowSavedModelBuilder::default()
    }

    fn export_dir(mut self, path: PathBuf) -> Self {
        self.export_dir = Some(path);
        self
    }

    fn model(mut self, bytes: Vec<u8>) -> Self {
        self.model = Some(bytes);
        self
    }

    fn checkpoint(mut self, bytes: Vec<u8>) -> Self {
        self.checkpoint = Some(bytes);
        self
    }

    fn var_index(mut self, bytes: Vec<u8>) -> Self {
        self.var_index = Some(bytes);
        self
    }

    fn build(self) -> Result<TensorflowSavedModel> {
        if let Some(path) = self.export_dir {
            return Ok(TensorflowSavedModel {
                export_dir: Some(path),
                ..Default::default()
            });
        }

        let model = self.model.ok_or(Error::InvalidArgument)?;
        let checkpoint = self.checkpoint.ok_or(Error::InvalidArgument)?;
        let var_index = self.var_index.ok_or(Error::InvalidArgument)?;

        Ok(TensorflowSavedModel {
            model: Some(model),
            checkpoint: Some(checkpoint),
            var_index: Some(var_index),
            ..Default::default()
        })
    }
}

impl ResourceType<'_> for TensorflowSavedModel {
    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TensorflowModel {
    id: u64,
    graph_path: PathBuf,
    graph_def: Option<Vec<u8>>,
}

impl ResourceType<'_> for TensorflowModel {
    fn id(&self) -> u64 {
        self.id
    }
}
