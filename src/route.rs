use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use router::{Router};
use serde_json;
use xitong::XiTong;
use zhiling::{ZhiLing, ZhiLingType};
use simctrl::{DevType, ZhanWeiType};

pub fn handler(_: &mut Request) -> IronResult<Response> {
    let x = XiTong::new(0);
    let x_ser_pretty = serde_json::to_string_pretty(&x).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, x_ser_pretty)))
}

pub fn zhi_ling_bei_che_handler(_: &mut Request) -> IronResult<Response> {
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

pub fn create_and_config_route() -> Router {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/zhiling/beiche", zhi_ling_bei_che_handler);
    return router;
}
