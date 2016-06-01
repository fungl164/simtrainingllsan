use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use std::io::prelude::*;
use serde_json;
use router::Router;

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

    pub fn create(req: &mut Request) -> IronResult<Response> {
        let mut data_json = String::new();
        let content_type_ok = "application/json".parse::<Mime>().unwrap();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<User>(&data_json) {
                    Ok(data) => {
                        let uid = data.uid.clone();
                        match users::table.find(uid).first::<User>(&(establish_connection())) {
                            Err(Error::NotFound) => {
                                match diesel::insert(&data)
                                          .into(users::table)
                                          .get_result::<User>(&(establish_connection())) {
                                    Ok(data) => {
                                        return Ok(Response::with((content_type_ok,
                                                                  status::Ok,
                                                                  serde_json::to_string(&data)
                                                                      .unwrap())))
                                    }
                                    Err(e) => {
                                        return Ok(Response::with((content_type_err,
                                                                  status::InternalServerError,
                                                                  format!("{:?}", e))))
                                    }
                                }
                            }
                            Ok(_) => {
                                return Ok(Response::with((content_type_err,
                                                          status::NotAcceptable,
                                                          "此记录已存在".to_string())))
                            }
                            Err(e) => {
                                return Ok(Response::with((content_type_err,
                                                          status::InternalServerError,
                                                          format!("{:?}", e))))
                            }
                        }
                    }
                    Err(e) => {
                        return Ok(Response::with((content_type_err,
                                                  status::BadRequest,
                                                  format!("{:?}", e))))
                    }
                }
            }
            Err(e) => {
                return Ok(Response::with((content_type_err,
                                          status::BadRequest,
                                          format!("{:?}", e))))
            }
        }
    }

    pub fn show_all(_: &mut Request) -> IronResult<Response> {
        match users::table.load::<User>(&(establish_connection())) {
            Ok(user_vec) => {
                let content_type_ok = "application/json".parse::<Mime>().unwrap();
                let user_vec_ser = serde_json::to_string(&user_vec).unwrap();
                return Ok(Response::with((content_type_ok, status::Ok, user_vec_ser)));
            }
            Err(e) => {
                let content_type_err = "text/plain".parse::<Mime>().unwrap();
                return Ok(Response::with((content_type_err, status::NotFound, format!("{:?}", e))));
            }
        }
    }

    pub fn find_by_id(req: &mut Request) -> IronResult<Response> {
        let uid = req.extensions.get::<Router>().unwrap().find("uid").unwrap();
        match users::table.find(uid.to_string()).first::<User>(&(establish_connection())) {
            Ok(data) => {
                let content_type_ok = "application/json".parse::<Mime>().unwrap();
                let data_ser = serde_json::to_string(&data).unwrap();
                return Ok(Response::with((content_type_ok, status::Ok, data_ser)));
            }
            Err(e) => {
                let content_type_err = "text/plain".parse::<Mime>().unwrap();
                return Ok(Response::with((content_type_err, status::NotFound, format!("{:?}", e))));
            }
        }
    }

    pub fn update(req: &mut Request) -> IronResult<Response> {
        use ::schema::users::dsl::*;
        let mut data_json = String::new();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        let _uid = req.extensions.get::<Router>().unwrap().find("uid").unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<User>(&data_json) {
                    Ok(data) => {
                        match diesel::update(users.filter(uid.eq(_uid.to_string())))
                                  .set(&data)
                                  .execute(&(establish_connection())) {
                            Ok(_) => {
                                match users.filter( uid.eq(_uid.to_string())).first::<User>(&(establish_connection()) ) {
                                    Ok(data) => {
                                        let content_type_ok = "application/json".parse::<Mime>().unwrap();
                                        let data_ser = serde_json::to_string(&data).unwrap();
                                        return Ok(Response::with((content_type_ok, status::Ok, data_ser)));
                                    }
                                    Err(e) => {
                                        return Ok(Response::with((content_type_err, status::NotFound, format!("{:?}", e))));
                                    }
                                }
                            }
                            Err(e) => {
                                return Ok(Response::with((content_type_err,
                                                          status::NotFound,
                                                          format!("{:?}", e))));
                            }
                        }
                    }
                    Err(e) => {
                        return Ok(Response::with((content_type_err,
                                                  status::BadRequest,
                                                  format!("{:?}", e))))
                    }
                }
            }
            Err(e) => {
                return Ok(Response::with((content_type_err,
                                          status::BadRequest,
                                          format!("{:?}", e))))
            }
        }
    }

    pub fn delete(req: &mut Request) -> IronResult<Response> {
        use ::schema::users::dsl::*;
        let content_type = "text/plain".parse::<Mime>().unwrap();
        let _uid = req.extensions.get::<Router>().unwrap().find("uid").unwrap();
        match diesel::delete(users.filter(uid.eq(_uid.to_string())))
                  .execute(&(establish_connection())) {
            Ok(n) if n>0 => {
                return Ok(Response::with((content_type,
                                          status::Ok,
                                          format!("成功删除{:?}条记录", n))));
            }
            Ok(_) => {
                return Ok(Response::with((content_type, status::NotFound, format!("{:?}", status::NotFound))));
            }
            Err(e) => {
                return Ok(Response::with((content_type, status::NotFound, format!("{:?}", e))));
            }
        }
    }
    
}
