use router::Router;
use mount::Mount;
use staticfile::Static;

use std::path::Path;
use user::User;
use zhanwei::ZhanWei;
use trainingsession::TrainingSession;
use trainingaction::TrainingAction;

pub fn build_route() -> Mount {
    let mut mount_api = Mount::new();
    mount_api.mount("/user/", build_user_route());
    mount_api.mount("/trainingsession/", build_training_session_route());
    mount_api.mount("/trainingaction/", build_training_action_route());
    mount_api.mount("/zhanwei/", build_zhan_wei_route());

    let mut mount_root = Mount::new();
    mount_root.mount("/api/v1/", mount_api);
    mount_root.mount("/docs/", Static::new(Path::new("target/doc")));
    return mount_root;
}

pub fn build_user_route() -> Router {
    let mut router = Router::new();
    router.get("/", User::show_all);
    router.get("/:uid", User::find_by_id);
    router.post("/", User::create);
    router.put("/:uid", User::update);
    router.delete("/:uid", User::delete);
    return router;
}

pub fn build_training_session_route() -> Router {
    let mut router = Router::new();
    router.get("/", TrainingSession::show_all);
    router.get("/:id", TrainingSession::find_by_id);
    router.post("/", TrainingSession::create);
    router.put("/:id", TrainingSession::update);
    router.delete("/:id", TrainingSession::delete);
    return router;
}

pub fn build_training_action_route() -> Router {
    let mut router = Router::new();
    router.get("/", TrainingAction::show_all);
    router.get("/:id", TrainingAction::find_by_id);
    router.post("/", TrainingAction::create);
    router.put("/:id", TrainingAction::update);
    router.delete("/:id", TrainingAction::delete);
    return router;
}

pub fn build_zhan_wei_route() -> Router {
    let mut router = Router::new();
    router.get("/", ZhanWei::show_all);
    router.get("/:uid", ZhanWei::find_by_id);
    router.post("/", ZhanWei::create);
    router.put("/:uid", ZhanWei::update);
    router.delete("/:uid", ZhanWei::delete);
    return router;
}
