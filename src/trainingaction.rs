use schema::*;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[insertable_into(training_actions)]
pub struct NewTrainingAction {
    pub sec: i64,
    pub nsec: i32,
    pub name: String,
    pub session_id: i32,
    pub user_uid: String,
    pub action_type: String,
    pub dev_uid: String,
    pub zhanwei_uid: String,
    pub sec_duration: i64,
    pub nsec_duration: i32,
    pub score_op_order: f64,
    pub score_op_correct: f64,
    pub score_op_duration: f64,
    pub score: f64,
}
impl NewTrainingAction {
    pub fn new(sec: i64,
               nsec: i32,
               name: &str,
               session_id: i32,
               user_uid: &str,
               action_type: &str,
               dev_uid: &str,
               zhanwei_uid: &str,
               sec_duration: i64,
               nsec_duration: i32,
               score_op_order: f64,
               score_op_correct: f64,
               score_op_duration: f64,
               score: f64)
               -> NewTrainingAction {
        NewTrainingAction {
            sec: sec,
            nsec: nsec,
            name: name.to_string(),
            session_id: session_id,
            user_uid: user_uid.to_string(),
            action_type: action_type.to_string(),
            dev_uid: dev_uid.to_string(),
            zhanwei_uid: zhanwei_uid.to_string(),
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
#[changeset_for(training_actions)]
pub struct TrainingAction {
    pub id: i32,
    pub sec: i64,
    pub nsec: i32,
    pub name: String,
    pub session_id: i32,
    pub user_uid: String,
    pub action_type: String,
    pub dev_uid: String,
    pub zhanwei_uid: String,
    pub sec_duration: i64,
    pub nsec_duration: i32,
    pub score_op_order: f64,
    pub score_op_correct: f64,
    pub score_op_duration: f64,
    pub score: f64,
}

impl TrainingAction {
    pub fn new(id: i32,
               sec: i64,
               nsec: i32,
               name: &str,
               session_id: i32,
               user_uid: &str,
               action_type: &str,
               dev_uid: &str,
               zhanwei_uid: &str,
               sec_duration: i64,
               nsec_duration: i32,
               score_op_order: f64,
               score_op_correct: f64,
               score_op_duration: f64,
               score: f64)
               -> TrainingAction {
        TrainingAction {
            id: id,
            sec: sec,
            nsec: nsec,
            name: name.to_string(),
            session_id: session_id,
            user_uid: user_uid.to_string(),
            action_type: action_type.to_string(),
            dev_uid: dev_uid.to_string(),
            zhanwei_uid: zhanwei_uid.to_string(),
            sec_duration: sec_duration,
            nsec_duration: nsec_duration,
            score_op_order: score_op_order,
            score_op_correct: score_op_correct,
            score_op_duration: score_op_duration,
            score: score,
        }
    }
    pub fn create(conn: &PgConnection,
                  action: NewTrainingAction)
                  -> diesel::QueryResult<TrainingAction> {
        diesel::insert(&action).into(training_actions::table).get_result(conn)
    }
    pub fn show_all(conn: &PgConnection) -> diesel::QueryResult<Vec<TrainingAction>> {
        training_actions::table.load::<TrainingAction>(conn)
    }
    pub fn find_by_id(conn: &PgConnection, _id: i32) -> diesel::QueryResult<TrainingAction> {
        training_actions::table.find(_id).first::<TrainingAction>(conn)
    }
    pub fn update(conn: &PgConnection,
                  _id: i32,
                  data: &TrainingAction)
                  -> diesel::QueryResult<usize> {
        use ::schema::training_actions::dsl::*;
        diesel::update(training_actions.filter(id.eq(_id))).set(data).execute(conn)
    }
    pub fn delete(conn: &PgConnection, _id: i32) -> diesel::QueryResult<usize> {
        use ::schema::training_actions::dsl::*;
        diesel::delete(training_actions.filter(id.eq(_id))).execute(conn)
    }
}
