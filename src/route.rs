use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::middleware::Handler;
use router::Router;
use mount::Mount;
use staticfile::Static;

use std::path::Path;
use serde_json;
use xitong::XiTong;
use zhiling::{ZhiLing, ZhiLingType};
use simctrl::{DevType, ZhanWeiType};

pub const API_ROOT_PATH: &'static str = "/api/v1";

pub fn xi_tong_handler(_: &mut Request) -> IronResult<Response> {
    let x = XiTong::new(0);
    let x_ser_pretty = serde_json::to_string_pretty(&x).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, x_ser_pretty)))
}

pub fn zhi_ling_handler(_req: &mut Request) -> IronResult<Response> {
    let mut z = ZhiLing::new();
    z.zhi_ling_type = ZhiLingType::BeiChe;
    z.dev_type = DevType::JiZu;
    z.dev_id = 0;
    z.actor_id = 0;
    z.zhan_wei_id = 0;
    z.zhan_wei_type = ZhanWeiType::JiaoLian;
    let mut x = XiTong::new(0);
    x.handle_zhi_ling(&z);
    let z_ser_pretty = serde_json::to_string(&z).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, z_ser_pretty)))
}
fn config_api_route<H, S>(router: &mut Router, path: S, handler: H) where H: Handler, S: AsRef<str> {
    router.get(API_ROOT_PATH.to_string() + path.as_ref(), handler);
}
pub fn create_and_config_route() -> Mount {
    let mut router = Router::new();
    config_api_route(&mut router, "/", xi_tong_handler);
    config_api_route(&mut router, "/zhiling", zhi_ling_handler);
    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/docs/", Static::new(Path::new("target/doc")));
    return mount;
}
