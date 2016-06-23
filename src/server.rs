use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use serde_json;
use router::Router;
use mount::Mount;
use iron::middleware::Handler;
use xitong::XiTong;
use route;
// use route::*;

pub struct Server {
    xi_tong_vec : Vec<XiTong>,
    pub iron : Iron<Mount>,
}
impl Server {
    pub fn new() -> Server {
        let mut router = Router::new();
        router.get("/", xi_tong_handler);
        let mut mount_root = route::build_route();
        mount_root.mount("/api/v1/xitong/", router);
        let mut s = Server {
            xi_tong_vec : Vec::new(),
            iron : Iron::new(mount_root),
        };
        s.xi_tong_vec.push(XiTong::new(0));
        fn xi_tong_handler(_req: &mut Request) -> IronResult<Response> {
            s.xi_tong_vec[0].query_xi_tong(_req);
        }
        return s;
    }
}
