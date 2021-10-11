#[allow(dead_code)]
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Session {
    /// Unique identifier of the session
    id: u64,
    /// Rundir for the session
    rundir: PathBuf,
}

impl Session {
    pub fn new() -> Self {
        Session::default()
    }

    pub(crate) fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub(crate) fn with_rundir(mut self, rundir: PathBuf) -> Self {
        self.rundir = rundir;
        self
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn rundir(&self) -> &Path {
        &self.rundir
    }
}
