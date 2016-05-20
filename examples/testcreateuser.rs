// #![feature(custom_derive, custom_attribute, plugin)]
// #![plugin(serde_macros, diesel_codegen, dotenv_macros)]
extern crate simtraining;
extern crate diesel;

use simtraining::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("请输入用户名:");
    let mut uid = String::new();
    stdin().read_line(&mut uid).unwrap();
    let uid = &uid[..(uid.len() - 1)]; // Drop the newline character

    println!("请输入密码:");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)]; // Drop the newline character

    println!("请输入真实姓名:");
    let mut realname = String::new();
    stdin().read_line(&mut realname).unwrap();
    let realname = &realname[..(realname.len() - 1)]; // Drop the newline character

    println!("请输入年龄:");
    let mut age = String::new();
    stdin().read_line(&mut age).unwrap();
    let age = &age[..(age.len() - 1)].parse::<i32>().unwrap(); // Drop the newline character

    let result = user::User::create(&connection, user::User::new(&uid, &password, 0, &realname, *age, ""));
    match result {
        Ok(user) => println!("用户{:?}创建成功", user),
        Err(err) => println!("{:?}", err),
    }
}
