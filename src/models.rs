use diesel::prelude::*;
use diesel::pg::data_types::PgTimestamp;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask<'a> {
    pub message: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTaskWithDate<'a> {
    pub message: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
