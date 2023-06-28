use diesel::prelude::*;
use diesel::mysql::data_types::MysqlTime;
    
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::film)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Film {
    pub film_id: u16,
    pub title: String,
    pub description: String,
    pub rental_rate: f32,
    pub last_update: MysqlTime
}
