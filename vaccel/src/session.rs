#[allow(dead_code)]
use std::path::{Path, PathBuf};

use crate::client::Vaccel;

#[derive(Debug, Default)]
pub struct Session<'a> {
    /// Unique identifier of the session
    id: u64,
    /// Rundir for the session
    rundir: Option<PathBuf>,
    /// Client used to create session
    client: Option<&'a Vaccel>,
}

impl<'a> Session<'a> {
    pub fn new() -> Self {
        Session::default()
    }

    pub(crate) fn with_client(mut self, client: &'a Vaccel) -> Self {
        self.client = Some(client);
        self
    }

    pub(crate) fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub(crate) fn with_rundir(mut self, rundir: PathBuf) -> Self {
        self.rundir = Some(rundir);
        self
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn rundir(&self) -> Option<&Path> {
        match self.rundir {
            None => None,
            Some(path) => Some(&path),
        }
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        if let Some(rundir) = self.rundir {
            let _ = std::fs::remove_dir(&rundir);
        }
    }
}
