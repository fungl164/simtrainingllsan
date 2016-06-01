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

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[changeset_for(zhanweis, treat_none_as_null="true")]
#[insertable_into(zhanweis)]
pub struct ZhanWei {
    pub uid: String,
    pub zhanwei_type: String,
    pub user_uid: Option<String>,
}

impl ZhanWei {
    pub fn new(uid: &str, zhanwei_type: &str, user_uid: Option<String>) -> ZhanWei {
        ZhanWei {
            uid: uid.to_string(),
            zhanwei_type: zhanwei_type.to_string(),
            user_uid: user_uid,
        }
    }
    pub fn create(req: &mut Request) -> IronResult<Response> {
        let mut data_json = String::new();
        let content_type_ok = "application/json".parse::<Mime>().unwrap();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<ZhanWei>(&data_json) {
                    Ok(data) => {
                        let uid = data.uid.clone();
                        match zhanweis::table.find(uid).first::<ZhanWei>(&(establish_connection())) {
                            Err(Error::NotFound) => {
                                match diesel::insert(&data)
                                          .into(zhanweis::table)
                                          .get_result::<ZhanWei>(&(establish_connection())) {
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
        match zhanweis::table.load::<ZhanWei>(&(establish_connection())) {
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
        match zhanweis::table.find(uid.to_string()).first::<ZhanWei>(&(establish_connection())) {
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
        use ::schema::zhanweis::dsl::*;
        let mut data_json = String::new();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        let _uid = req.extensions.get::<Router>().unwrap().find("uid").unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<ZhanWei>(&data_json) {
                    Ok(data) => {
                        match diesel::update(zhanweis.filter(uid.eq(_uid.to_string())))
                                  .set(&data)
                                  .execute(&(establish_connection())) {
                            Ok(_) => {
                                match zhanweis.filter( uid.eq(_uid.to_string())).first::<ZhanWei>(&(establish_connection()) ) {
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
        use ::schema::zhanweis::dsl::*;
        let content_type = "text/plain".parse::<Mime>().unwrap();
        let _uid = req.extensions.get::<Router>().unwrap().find("uid").unwrap();
        match diesel::delete(zhanweis.filter(uid.eq(_uid.to_string())))
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
