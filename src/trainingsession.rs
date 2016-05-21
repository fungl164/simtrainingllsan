use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
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
    pub fn create(conn: &PgConnection,
                  _session: NewTrainingSession)
                  -> diesel::QueryResult<TrainingSession> {
        diesel::insert(&_session).into(training_sessions::table).get_result(conn)
    }
    pub fn show_all(conn: &PgConnection) -> diesel::QueryResult<Vec<TrainingSession>> {
        training_sessions::table.load::<TrainingSession>(conn)
    }
    pub fn find_by_id(conn: &PgConnection, _id: i32) -> diesel::QueryResult<TrainingSession> {
        training_sessions::table.find(_id).first::<TrainingSession>(conn)
    }
    pub fn update(conn: &PgConnection,
                  _id: i32,
                  data: &TrainingSession)
                  -> diesel::QueryResult<usize> {
        use ::schema::training_sessions::dsl::*;
        diesel::update(training_sessions.filter(id.eq(_id))).set(data).execute(conn)
    }
    pub fn delete(conn: &PgConnection, _id: i32) -> diesel::QueryResult<usize> {
        use ::schema::training_sessions::dsl::*;
        diesel::delete(training_sessions.filter(id.eq(_id))).execute(conn)
    }
}
