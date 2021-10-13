use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use dashmap::DashMap;

use tarpc::context::Context;

use mktemp::Temp;

use crate::resource::Resource;
use crate::session::Session;
use crate::{Error, Result};

#[tarpc::service]
pub trait VaccelAPI {
    /// Create a new vAccel session
    async fn new_session() -> Result<u64>;

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
    pub fn new() -> Result<Self> {
        let vaccel_path =
            Path::new(&format!("/run/user/{}/vaccel", users::get_current_uid())).to_path_buf();

        // If vAccel rundir path does not exist, create it
        if !vaccel_path.is_dir() {
            fs::create_dir(&vaccel_path)?;
        }

        let rundir = Temp::new_dir_in(&Path::new(&vaccel_path)).map_err(|_| Error::IOError)?;

        Ok(Server {
            0: Arc::new(ServerState {
                rundir,
                session_id: AtomicU64::new(1),
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
impl VaccelAPI for Server {
    async fn new_session(self, _: Context) -> Result<u64> {
        let id = self.next_id();
        let mut rundir = self.0.rundir.as_path().to_path_buf();

        rundir.push(format!("session.{}", id));
        fs::create_dir(&rundir)?;

        let session = Session::new()
            .with_id(id)
            .with_rundir(rundir.as_path().to_path_buf());
        self.0.sessions.insert(id, Arc::new(session));

        Ok(id)
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
