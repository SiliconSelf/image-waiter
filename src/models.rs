use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Image {
    pub(crate) id: i32,
    pub(crate) url: String
}