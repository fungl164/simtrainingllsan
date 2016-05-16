// #![feature(custom_derive, custom_attribute, plugin)]
// #![plugin(serde_macros, diesel_codegen, dotenv_macros)]
extern crate simtraining;
extern crate diesel;

use simtraining::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("请输入要删除的用户名:");
    let mut uid = String::new();
    stdin().read_line(&mut uid).unwrap();
    let uid = &uid[..(uid.len() - 1)]; // Drop the newline character

    match user::User::delete(&connection, &uid) {
        Ok(num) => println!("成功删除{:?}条记录", num),
        Err(err) => println!("{:?}", err),
    }
}
