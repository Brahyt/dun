use diesel::prelude::*;
use diesel::pg::data_types::PgTimestamp;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub message: String,
}
