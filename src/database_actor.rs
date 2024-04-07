//! Actor that provides database connection functionality

use diesel::{Connection, PgConnection, RunQueryDsl};

use crate::{configuration::APP_CONFIG, models::Image};

/// An actor that provides various database functions
pub(crate) struct DatabaseActor {
    /// The connection to the postgres database
    _connection: PgConnection,
    /// The images retrieved from the database at startup
    images: Vec<Image>,
}

impl DatabaseActor {
    /// Create a new database actor
    pub(crate) fn new() -> Self {
        use crate::schema::images::dsl::images;

        let mut connection = establish_connection();
        let results: Vec<Image> =
            images.load(&mut connection).expect("Error loading images");

        Self {
            _connection: connection,
            images: results,
        }
    }

    /// Return a reference to the internal `images`
    pub(crate) fn get_images(&self) -> &Vec<Image> {
        &self.images
    }
}

/// Establish a connection to the database
fn establish_connection() -> PgConnection {
    PgConnection::establish(APP_CONFIG.get_database_url())
        .expect("Failed to connect to postgres database")
}
