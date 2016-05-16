use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    Unisex,
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[changeset_for(users)]
#[insertable_into(users)]
pub struct User {
    pub uid: String,
    pub realname: String,
    pub age: i32,
    pub sex: String,
}

impl User {
    pub fn new(_id: &str, _realname: &str, _age: i32, _sex: &str) -> User {
        User {
            uid: _id.to_string(),
            realname: _realname.to_string(),
            age: _age,
            sex: _sex.to_string(),
        }
    }
    pub fn create<'a>(conn: &PgConnection,
                      _uid: &'a str,
                      _realname: &'a str,
                      _age: i32,
                      _sex: &'a str)
                      -> diesel::QueryResult<User> {
        let uid_s = String::from(_uid);

        match users::table.find(uid_s).first::<User>(conn) {
            Err(Error::NotFound) => {
                let new_user = User::new(_uid, _realname, _age, _sex);
                return ::diesel::insert(&new_user).into(users::table)
                    .get_result(conn);
            }
            Ok(_) => return Err(Error::DatabaseError("此用户名已存在".to_string())),
            Err(e) => return Err(e),
        }
    }
    pub fn show_all(conn: &PgConnection) -> diesel::QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }
    pub fn find_by_id(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<User> {
        users::table.find(_uid.to_string()).first::<User>(conn)
    }
    pub fn update(conn: &PgConnection, _uid: &str, data: &User) -> diesel::QueryResult<usize> {
        use ::schema::users::dsl::*;
        diesel::update(users.filter(uid.eq(_uid.to_string()))).set(data).execute(conn)
    }
    pub fn delete(conn: &PgConnection, _uid: &str) -> diesel::QueryResult<usize> {
        use ::schema::users::dsl::*;
        diesel::delete(users.filter(uid.eq(_uid.to_string()))).execute(conn)
    }

}
