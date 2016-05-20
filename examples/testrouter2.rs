extern crate simtraining;
extern crate iron;
extern crate router;

use iron::prelude::*;
use simtraining::route;

fn main() {
    Iron::new(route::create_and_config_route()).http("localhost:3000").unwrap();
}
