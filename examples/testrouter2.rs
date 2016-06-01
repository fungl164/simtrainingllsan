extern crate simtraining;
extern crate iron;
extern crate router;

use iron::prelude::*;
use simtraining::route;

fn main() {
    Iron::new(route::build_route()).http("localhost:3000").unwrap();
}
