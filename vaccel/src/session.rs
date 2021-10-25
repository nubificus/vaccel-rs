#[allow(dead_code)]
use std::path::{Path, PathBuf};

use log::debug;

#[derive(Debug, Default)]
pub struct Session {
    /// Unique identifier of the session
    id: u64,
    /// Rundir for the session
    rundir: Option<PathBuf>,
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
        self.rundir = Some(rundir);
        self
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn rundir(&self) -> Option<&Path> {
        match &self.rundir {
            None => None,
            Some(path) => Some(&path),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        debug!("Dropping session: {}", self.id());
        if let Some(ref rundir) = self.rundir {
            let _ = std::fs::remove_dir(&rundir);
        }
    }
}
