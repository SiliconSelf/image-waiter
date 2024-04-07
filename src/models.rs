//! Database models used by diesel

use diesel::prelude::*;

/// An image row
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Image {
    /// The id of the image
    pub(crate) id: i32,
    /// The url of the image in the CDN
    pub(crate) url: String,
}
