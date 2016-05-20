use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[changeset_for(zhanweis, treat_none_as_null="true")]
#[insertable_into(zhanweis)]
pub struct ZhanWei {
    pub uid: String,
    pub zhanwei_type: String,
    pub user_id: Option<String>,
}

impl ZhanWei {
    pub fn new(uid: &str, zhanwei_type: &str, user_id: Option<String>) -> ZhanWei {
        ZhanWei {
            uid: uid.to_string(),
            zhanwei_type: zhanwei_type.to_string(),
            user_id: user_id,
        }
    }
    pub fn create(conn: &PgConnection, _zhanwei: ZhanWei) -> diesel::QueryResult<ZhanWei> {
        let zhanwei = _zhanwei.clone();
        match zhanweis::table.find(_zhanwei.uid).first::<ZhanWei>(conn) {
            Err(Error::NotFound) => {
                return ::diesel::insert(&zhanwei)
                           .into(zhanweis::table)
                           .get_result(conn);
            }
            Ok(_) => return Err(Error::DatabaseError("此记录已存在".to_string())),
            Err(e) => return Err(e),
        }
    }
    pub fn show_all(conn: &PgConnection) -> diesel::QueryResult<Vec<ZhanWei>> {
        zhanweis::table.load::<ZhanWei>(conn)
    }
    pub fn find_by_id(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<ZhanWei> {
        zhanweis::table.find(_uid.to_string()).first::<ZhanWei>(conn)
    }
    pub fn update(conn: &PgConnection, _uid: &str, data: &ZhanWei) -> diesel::QueryResult<usize> {
        use ::schema::zhanweis::dsl::*;
        diesel::update(zhanweis.filter(uid.eq(_uid.to_string()))).set(data).execute(conn)
    }
    pub fn delete(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<usize> {
        use ::schema::zhanweis::dsl::*;
        diesel::delete(zhanweis.filter(uid.eq(_uid.to_string()))).execute(conn)
    }
}
