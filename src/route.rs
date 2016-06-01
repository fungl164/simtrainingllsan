use router::Router;
use mount::Mount;
use staticfile::Static;

use std::path::Path;
use user::User;
pub fn build_route() -> Mount {
    let mut mount_api = Mount::new();
    mount_api.mount("/user/", build_user_route());

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
