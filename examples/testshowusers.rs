// #![feature(custom_derive, custom_attribute, plugin)]
// #![plugin(serde_macros, diesel_codegen, dotenv_macros)]
extern crate simtraining;
extern crate diesel;

use simtraining::schema::establish_connection;
use simtraining::user;

fn main() {
    let connection = establish_connection();

    let results = user::User::show_all();
    match results {
        Ok(users) => {
            println!("显示 {} 个用户", users.len());
            println!("{:?}", users);
        },
        Err(err) => println!("{:?}", err),
    }
}
