use diesel;
use diesel::result::Error;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::users;
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    Unisex,
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub uid: String,
    pub realname: String,
    pub age: i32,
    pub sex: String,
}

#[insertable_into(users)]
pub struct NewUser<'a> {
    pub uid: &'a str,
    pub realname: &'a str,
    //pub age: i32,
    pub sex: &'a str,
}

impl User {
    pub fn new(_id: &str, _realname: &str) -> User {
        User {
            uid: String::from(_id),
            realname: String::from(_realname),
            age: 0,
            sex: "".to_string(),
        }
    }

    pub fn create_user<'a>(conn: &PgConnection,
                      _uid: &'a str,
                      _realname: &'a str,
                      _age: i32,
                      _sex: &'a str)
                      -> QueryResult<User> {
        let uid_s = String::from(_uid);

        let result = users::table.find(uid_s).first(conn);
        match result {
            Err(NotFound) => {
                let new_user = User {
                    uid: String::from(_uid),
                    realname: String::from(_realname),
                    age : _age,
                    sex : _sex.to_string(),
                };
                return diesel::insert(&new_user).into(users::table)
                    .get_result(conn);
            }
            Ok(_) => return Err(Error::DatabaseError("此用户名已存在".to_string())),
        }
    }

}
