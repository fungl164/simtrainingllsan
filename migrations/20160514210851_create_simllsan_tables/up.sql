CREATE TABLE users (
    uid VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL,
    level INTEGER NOT NULL,
    realname VARCHAR NOT NULL,
    age INTEGER NOT NULL,
    sex VARCHAR NOT NULL
);
CREATE TABLE training_sessions (
  id SERIAL PRIMARY KEY,
  sec BIGINT NOT NULL,
  nsec INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  xitong_id INTEGER NOT NULL,
  admin_uid VARCHAR NOT NULL,
  users_uid VARCHAR[] NOT NULL,
  actions_id INTEGER[] NOT NULL,
  mode VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  sec_duration BIGINT NOT NULL,
  nsec_duration INTEGER NOT NULL,
  score_op_order DOUBLE PRECISION NOT NULL,
  score_op_correct DOUBLE PRECISION NOT NULL,
  score_op_duration DOUBLE PRECISION NOT NULL,
  score DOUBLE PRECISION NOT NULL
);

CREATE TABLE training_actions (
  id SERIAL PRIMARY KEY,
  sec BIGINT NOT NULL,
  nsec INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  session_id INTEGER NOT NULL,
  user_uid INTEGER NOT NULL,
  action_type VARCHAR NOT NULL,
  dev_uid VARCHAR NOT NULL,
  zhanwei_uid VARCHAR NOT NULL,
  sec_duration BIGINT NOT NULL,
  nsec_duration INTEGER NOT NULL,
  score_op_order DOUBLE PRECISION NOT NULL,
  score_op_correct DOUBLE PRECISION NOT NULL,
  score_op_duration DOUBLE PRECISION NOT NULL,
  score DOUBLE PRECISION NOT NULL
);

CREATE TABLE zhanweis (
  uid VARCHAR PRIMARY KEY,
  zhanwei_type VARCHAR NOT NULL,
  user_id VARCHAR
);

CREATE TABLE devs (
  uid VARCHAR PRIMARY KEY,
  id INTEGER NOT NULL,
  dev_type VARCHAR NOT NULL
);
