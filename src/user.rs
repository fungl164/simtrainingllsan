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
    pub password: String,
    pub level: i32,
    pub realname: String,
    pub age: i32,
    pub sex: String,
}

impl User {
    pub fn new(_uid: &str,
               _password: &str,
               _level: i32,
               _realname: &str,
               _age: i32,
               _sex: &str)
               -> User {
        User {
            uid: _uid.to_string(),
            password: _password.to_string(),
            level: _level,
            realname: _realname.to_string(),
            age: _age,
            sex: _sex.to_string(),
        }
    }
    pub fn create(conn: &PgConnection, _user: User) -> diesel::QueryResult<User> {
        let user = _user.clone();
        match users::table.find(_user.uid).first::<User>(conn) {
            Err(Error::NotFound) => {
                return ::diesel::insert(&user)
                           .into(users::table)
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
