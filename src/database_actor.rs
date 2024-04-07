//! Actor that provides database connection functionality

use diesel::{Connection, PgConnection};

use crate::configuration::APP_CONFIG;

/// An actor that provides various database functions
pub(crate) struct DatabaseActor {
    images: Vec<DatabaseImage>,
}

impl DatabaseActor {
    /// Create a new database actor
    pub(crate) fn new() -> Self {
        PgConnection::establish(APP_CONFIG.get_database_path())
            .expect("Failed to connect to postgres database");

        Self {
            images: Vec::new(),
        }
    }
}

pub(crate) struct DatabaseImage;
