// #![feature(custom_derive, custom_attribute, plugin)]
// #![plugin(serde_macros, diesel_codegen, dotenv_macros)]
extern crate simtraining;
extern crate diesel;

use simtraining::*;

fn main() {
    let connection = establish_connection();
    match user::User::find_by_id(&connection, "浮影") {
        Ok(_) => {
            let user = user::User::new("浮影", "", 0, "男");
            match user::User::update(&connection, "浮影", &user) {
                Ok(num) => println!("成功更新{:?}条记录", num),
                Err(err) => println!("{:?}", err),
            }
        }
        Err(err) => println!("查询用户浮影失败: {:?}", err),
    }
}
