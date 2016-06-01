use schema::*;
use diesel;
use diesel::prelude::*;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use std::io::prelude::*;
use serde_json;
use router::Router;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[insertable_into(training_sessions)]
pub struct NewTrainingSession {
    pub sec: i64,
    pub nsec: i32,
    pub name: String,
    pub xitong_id: i32,
    pub admin_uid: String,
    pub users_uid: Vec<String>,
    pub actions_id: Vec<i32>,
    pub mode: String,
    pub state: String,
    pub sec_duration: i64,
    pub nsec_duration: i32,
    pub score_op_order: f64,
    pub score_op_correct: f64,
    pub score_op_duration: f64,
    pub score: f64,
}
impl NewTrainingSession {
    pub fn new(sec: i64,
               nsec: i32,
               name: &str,
               xitong_id: i32,
               admin_uid: &str,
               users_uid: Vec<String>,
               actions_id: Vec<i32>,
               mode: &str,
               state: &str,
               sec_duration: i64,
               nsec_duration: i32,
               score_op_order: f64,
               score_op_correct: f64,
               score_op_duration: f64,
               score: f64)
               -> NewTrainingSession {
        NewTrainingSession {
            sec: sec,
            nsec: nsec,
            name: name.to_string(),
            xitong_id: xitong_id,
            admin_uid: admin_uid.to_string(),
            users_uid: users_uid,
            actions_id: actions_id,
            mode: mode.to_string(),
            state: state.to_string(),
            sec_duration: sec_duration,
            nsec_duration: nsec_duration,
            score_op_order: score_op_order,
            score_op_correct: score_op_correct,
            score_op_duration: score_op_duration,
            score: score,
        }
    }
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Queryable)]
#[changeset_for(training_sessions)]
pub struct TrainingSession {
    pub id: i32,
    pub sec: i64,
    pub nsec: i32,
    pub name: String,
    pub xitong_id: i32,
    pub admin_uid: String,
    pub users_uid: Vec<String>,
    pub actions_id: Vec<i32>,
    pub mode: String,
    pub state: String,
    pub sec_duration: i64,
    pub nsec_duration: i32,
    pub score_op_order: f64,
    pub score_op_correct: f64,
    pub score_op_duration: f64,
    pub score: f64,
}

impl TrainingSession {
    pub fn new(id: i32,
               sec: i64,
               nsec: i32,
               name: &str,
               xitong_id: i32,
               admin_uid: &str,
               users_uid: Vec<String>,
               actions_id: Vec<i32>,
               mode: &str,
               state: &str,
               sec_duration: i64,
               nsec_duration: i32,
               score_op_order: f64,
               score_op_correct: f64,
               score_op_duration: f64,
               score: f64)
               -> TrainingSession {
        TrainingSession {
            id: id,
            sec: sec,
            nsec: nsec,
            name: name.to_string(),
            xitong_id: xitong_id,
            admin_uid: admin_uid.to_string(),
            users_uid: users_uid,
            actions_id: actions_id,
            mode: mode.to_string(),
            state: state.to_string(),
            sec_duration: sec_duration,
            nsec_duration: nsec_duration,
            score_op_order: score_op_order,
            score_op_correct: score_op_correct,
            score_op_duration: score_op_duration,
            score: score,
        }
    }

    pub fn create(req: &mut Request) -> IronResult<Response> {
        let mut data_json = String::new();
        let content_type_ok = "application/json".parse::<Mime>().unwrap();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<NewTrainingSession>(&data_json) {
                    Ok(data) => {
                        match diesel::insert(&data)
                                  .into(training_sessions::table)
                                  .get_result::<TrainingSession>(&(establish_connection())) {
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
        match training_sessions::table.load::<TrainingSession>(&(establish_connection())) {
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
        let id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
        match training_sessions::table.find(id).first::<TrainingSession>(&(establish_connection())) {
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
        use ::schema::training_sessions::dsl::*;
        let mut data_json = String::new();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        let _id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<TrainingSession>(&data_json) {
                    Ok(data) => {
                        match diesel::update(training_sessions.filter(id.eq(_id)))
                                  .set(&data)
                                  .execute(&(establish_connection())) {
                            Ok(_) => {
                                match training_sessions.filter( id.eq(_id)).first::<TrainingSession>(&(establish_connection()) ) {
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
        use ::schema::training_sessions::dsl::*;
        let content_type = "text/plain".parse::<Mime>().unwrap();
        let _id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
        match diesel::delete(training_sessions.filter(id.eq(_id)))
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
