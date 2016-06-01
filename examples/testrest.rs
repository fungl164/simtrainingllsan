extern crate simtraining;
extern crate iron;
extern crate rustless;

use simtraining::route;
fn main() {
    let mut chain = iron::Chain::new(route::test_rest_api());
    chain.link(::rustless::batteries::cookie::new("secretsecretsecretsecretsecretsecretsecret".as_bytes()));

    iron::Iron::new(chain).http("localhost:4000").unwrap();
}
