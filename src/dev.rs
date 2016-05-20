use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[changeset_for(devs)]
#[insertable_into(devs)]
pub struct Dev {
    pub uid: String,
    pub id: i32,
    pub dev_type: String,
}

impl Dev {
    pub fn new(uid: &str, id: i32, dev_type: &str) -> Dev {
        Dev {
            uid: uid.to_string(),
            id: id,
            dev_type: dev_type.to_string(),
        }
    }
    pub fn create(conn: &PgConnection, _dev: Dev) -> diesel::QueryResult<Dev> {
        let dev = _dev.clone();
        match devs::table.find(_dev.uid).first::<Dev>(conn) {
            Err(Error::NotFound) => {
                return ::diesel::insert(&dev)
                           .into(devs::table)
                           .get_result(conn);
            }
            Ok(_) => return Err(Error::DatabaseError("此记录已存在".to_string())),
            Err(e) => return Err(e),
        }
    }
    pub fn show_all(conn: &PgConnection) -> diesel::QueryResult<Vec<Dev>> {
        devs::table.load::<Dev>(conn)
    }
    pub fn find_by_id(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<Dev> {
        devs::table.find(_uid.to_string()).first::<Dev>(conn)
    }
    pub fn update(conn: &PgConnection, _uid: &str, data: &Dev) -> diesel::QueryResult<usize> {
        use ::schema::devs::dsl::*;
        diesel::update(devs.filter(uid.eq(_uid.to_string()))).set(data).execute(conn)
    }
    pub fn delete(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<usize> {
        use ::schema::devs::dsl::*;
        diesel::delete(devs.filter(uid.eq(_uid.to_string()))).execute(conn)
    }
}
