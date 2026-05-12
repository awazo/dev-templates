use std::sync::atomic::{AtomicU8, Ordering};

use crate::db::Db;

#[derive(Debug)]
pub struct WebState {
    pub db: Db,
    status: AtomicU8,
}

impl WebState {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            status: AtomicU8::new(Status::Starting as u8),
        }
    }

    pub fn set_status(&self, status: Status) {
        self.status.store(status as u8, Ordering::SeqCst);
    }

    pub fn is_ready(&self) -> bool {
        Status::from(&self.status).is_ready()
    }

    pub fn is_shutting_down(&self) -> bool {
        Status::from(&self.status).is_shutting_down()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Starting = 0,
    Running = 1,
    Draining = 2,
    Stopped = 3,
}

impl Status {
    pub fn is_ready(&self) -> bool {
        *self == Status::Running
    }

    pub fn is_shutting_down(&self) -> bool {
        *self as u8 >= Status::Draining as u8
    }
}

impl From<&AtomicU8> for Status {
    fn from(value: &AtomicU8) -> Self {
        match value.load(Ordering::SeqCst) {
            0 => Self::Starting,
            1 => Self::Running,
            2 => Self::Draining,
            3 => Self::Stopped,
            _ => Self::Stopped, // default to Stopped for invalid values
        }
    }
}
