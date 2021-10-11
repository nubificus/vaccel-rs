use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use dashmap::DashMap;

use tarpc::context::Context;

use mktemp::Temp;

use crate::resource::Resource;
use crate::session::Session;
use crate::{Error, Result};

#[tarpc::service]
pub trait Vaccel {
    /// Create a new vAccel session
    async fn new_session() -> u64;

    /// Destroy a vAccel session
    async fn destroy_session(session: u64) -> Result<()>;

    /// Register a new vAccel resource
    async fn register_resource(resource: Resource) -> u64;
}

#[derive(Clone)]
pub struct Server(Arc<ServerState>);

pub struct ServerState {
    rundir: mktemp::Temp,
    session_id: AtomicU64,
    sessions: DashMap<u64, Arc<Session>>,
}

impl Server {
    fn new() -> Result<Self> {
        let rundir = Temp::new_dir_in(&Path::new("/run/user")).map_err(|_| Error::IOError)?;

        Ok(Server {
            0: Arc::new(ServerState {
                rundir,
                session_id: AtomicU64::new(0),
                sessions: DashMap::new(),
            }),
        })
    }

    fn next_id(&self) -> u64 {
        self.0.session_id.fetch_add(1, Ordering::SeqCst)
    }

    fn remove_session(&self, session_id: &u64) -> Option<Arc<Session>> {
        match self.0.sessions.remove(session_id) {
            None => None,
            Some((_, session)) => Some(session),
        }
    }

    fn get_session(&self, session_id: &u64) -> Option<Arc<Session>> {
        self.0
            .sessions
            .get(session_id)
            .map(|r| Arc::clone(r.value()))
    }
}

#[tarpc::server]
impl Vaccel for Server {
    async fn new_session(self, _: Context) -> u64 {
        let id = self.next_id();
        let mut rundir = self.0.rundir.clone();
        rundir.push(format!("session.{}", id));

        let session = Session::default()
            .with_id(id)
            .with_rundir(rundir.as_path().to_path_buf());
        self.0.sessions.insert(id, Arc::new(session));

        id
    }

    async fn destroy_session(self, _: Context, session_id: u64) -> Result<()> {
        match self.remove_session(&session_id) {
            None => Err(Error::InvalidArgument),
            Some(_) => Ok(()),
        }
    }

    async fn register_resource(self, _: Context, _resource: Resource) -> u64 {
        todo!()
    }
}
