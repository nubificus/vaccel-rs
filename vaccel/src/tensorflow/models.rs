use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::resource::ResourceType;
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
struct InMemorySavedModel {
    model: Vec<u8>,
    checkpoint: Vec<u8>,
    var_index: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
enum SavedModel {
    ExportDir(PathBuf),
    InMemory(InMemorySavedModel),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TensorflowSavedModel {
    id: u64,
    model: SavedModel,
}

#[derive(Default)]
pub struct TensorflowSavedModelBuilder {
    export_dir: Option<PathBuf>,
    model: Option<Vec<u8>>,
    checkpoint: Option<Vec<u8>>,
    var_index: Option<Vec<u8>>,
}

impl TensorflowSavedModelBuilder {
    pub fn new() -> Self {
        TensorflowSavedModelBuilder::default()
    }

    pub fn export_dir(mut self, path: PathBuf) -> Self {
        self.export_dir = Some(path);
        self
    }

    pub fn model(mut self, bytes: Vec<u8>) -> Self {
        self.model = Some(bytes);
        self
    }

    pub fn checkpoint(mut self, bytes: Vec<u8>) -> Self {
        self.checkpoint = Some(bytes);
        self
    }

    pub fn var_index(mut self, bytes: Vec<u8>) -> Self {
        self.var_index = Some(bytes);
        self
    }

    pub fn build(self) -> Result<TensorflowSavedModel> {
        if let Some(path) = self.export_dir {
            return Ok(TensorflowSavedModel {
                id: 0,
                model: SavedModel::ExportDir(path),
            });
        }

        let model = InMemorySavedModel {
            model: self.model.ok_or(Error::InvalidArgument)?,
            checkpoint: self.checkpoint.ok_or(Error::InvalidArgument)?,
            var_index: self.var_index.ok_or(Error::InvalidArgument)?,
        };

        Ok(TensorflowSavedModel {
            id: 0,
            model: SavedModel::InMemory(model),
        })
    }
}

impl ResourceType for TensorflowSavedModel {
    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum ProtobufModel {
    Protobuf(PathBuf),
    InMemory(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TensorflowModel {
    id: u64,
    model: ProtobufModel,
}

impl ResourceType for TensorflowModel {
    fn id(&self) -> u64 {
        self.id
    }
}
